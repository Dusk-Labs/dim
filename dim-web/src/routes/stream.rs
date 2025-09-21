use crate::error::DimErrorWrapper;
use crate::AppState;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::response::Response;
use axum::Extension;
use dim_core::core::StateManager;

use dim_core::stream_tracking::ContentType;
use dim_core::stream_tracking::StreamTracking;
use dim_core::stream_tracking::VirtualManifest;
use dim_core::streaming::ffprobe::FFPStream;
use dim_core::streaming::ffprobe::FFProbeCtx;
use dim_core::streaming::get_avc1_tag;
use dim_core::streaming::get_qualities;
use dim_core::streaming::level_to_tag;
use dim_core::utils::quality_to_label;

use dim_database::mediafile::MediaFile;
use dim_database::user::DefaultVideoQuality;
use dim_database::user::User;
use dim_database::user::UserSettings;

use nightfall::error::NightfallError;
use nightfall::profiles::*;

use std::future::Future;
use std::path;
use std::path::PathBuf;
use std::time::Duration;

use futures::stream;
use futures::StreamExt;

use tokio::fs::File;

use serde::Deserialize;
use serde_json::json;

use http::header;
use http::StatusCode;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct VirtualManifestParams {
    gid: Option<String>,
    #[serde(default)]
    force_ass: bool,
}

/// Method mapped to `GET /api/v1/stream/<id>/manifest?<gid>` returns or creates a virtual
/// manifest.
pub async fn return_virtual_manifest(
    State(AppState {
        conn,
        state,
        stream_tracking,
        ..
    }): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<VirtualManifestParams>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, DimErrorWrapper> {
    let gid = params.gid.and_then(|x| Uuid::parse_str(x.as_str()).ok());
    if let Some(gid) = gid {
        return Ok(Json(&json!({
            "tracks": stream_tracking.get_for_gid(&gid).await,
            "gid": gid.as_hyphenated().to_string(),
        }))
        .into_response());
    }

    let mut tx = conn.read().begin().await?;
    let user_prefs = user.prefs;

    let gid = uuid::Uuid::new_v4();

    let media = MediaFile::get_one(&mut tx, id)
        .await
        .map_err(|e| dim_core::errors::StreamingErrors::NoMediaFileFound(e.to_string()))?;

    let target_file = media.target_file.clone();

    // FIXME: When `fs::try_exists` gets stabilized we should use that as it will allow us to
    // detect if the user lacks permissions to access the file, etc.
    if !path::Path::new(&target_file).exists() {
        return Err(dim_core::errors::StreamingErrors::FileDoesNotExist.into());
    }

    let info = FFProbeCtx::new(dim_core::streaming::FFPROBE_BIN.as_ref())
        .get_meta(target_file)
        .await
        .map_err(|_| dim_core::errors::StreamingErrors::FFProbeCtxFailed)?;

    let mut ms = info
        .get_ms()
        .ok_or(dim_core::errors::StreamingErrors::FileIsCorrupt)?
        .to_string();

    ms.truncate(4);

    let should_stream_default =
        try_create_dstream(&info, &media, &stream_tracking, &gid, &state, &user_prefs).await?;

    create_video(
        &info,
        &media,
        &stream_tracking,
        &gid,
        &state,
        &user_prefs,
        should_stream_default,
    )
    .await?;
    create_audio(&info, &media, &stream_tracking, &gid, &state).await?;
    create_subtitles(
        &info,
        &media,
        &stream_tracking,
        &gid,
        &state,
        params.force_ass,
    )
    .await?;

    stream_tracking.generate_sids(&gid).await;

    Ok(Json(&json!({
        "tracks": stream_tracking.get_for_gid(&gid).await,
        "gid": gid.as_hyphenated().to_string(),
    }))
    .into_response())
}

pub async fn try_create_dstream(
    info: &FFPStream,
    media: &MediaFile,
    stream_tracking: &StreamTracking,
    gid: &Uuid,
    state: &StateManager,
    prefs: &UserSettings,
) -> Result<bool, DimErrorWrapper> {
    let video_stream = info
        .get_primary("video")
        .cloned()
        .ok_or(dim_core::errors::StreamingErrors::FileIsCorrupt)?;

    let ctx = ProfileContext {
        file: media.target_file.clone(),
        input_ctx: video_stream.clone().into(),
        output_ctx: OutputCtx {
            codec: "h264".into(),
            start_num: 0,
            target_gop: 10,
            ..Default::default()
        },
        ..Default::default()
    };

    let dp_profile_chain =
        get_profile_for_with_type(StreamType::Video, ProfileType::Transmux, &ctx);

    // Should secondary (transcoded) streams default.
    let should_stream_default = dp_profile_chain.is_empty()
        || !matches!(prefs.default_video_quality, DefaultVideoQuality::DirectPlay);

    if !dp_profile_chain.is_empty() {
        let video = state.create(dp_profile_chain, ctx).await?;

        // FIXME: Stop hardcoding a fps of 24
        let video_avc = video_stream
            .level
            .and_then(|x| level_to_tag(x))
            .unwrap_or(get_avc1_tag(
                video_stream.width.clone().unwrap_or(1920) as u64,
                video_stream.height.clone().unwrap_or(1080) as u64,
                video_stream
                    .get_bitrate()
                    .or(info.get_container_bitrate())
                    .expect("Failed to pick bitrate for video stream"),
                24,
            ));

        let bitrate = video_stream
            .get_bitrate()
            .or(info.get_container_bitrate())
            .unwrap_or(10_000_000);

        let label = {
            let (ident, bitrate_norm) = if bitrate > 1_000_000 {
                ("MB", bitrate / 1_000_000)
            } else {
                ("KB", bitrate / 1_000)
            };

            format!(
                "{}p@{}{} (Direct Play)",
                video_stream.height.clone().unwrap(),
                bitrate_norm,
                ident
            )
        };

        let chunk_path = format!("{}/data/$Number$.m4s", video.clone());
        let init_seg = Some(format!("{}/data/init.mp4", video.clone()));
        let virtual_manifest =
            VirtualManifest::new(video.clone(), chunk_path, init_seg, ContentType::Video)
                .set_direct()
                .set_mime("video/mp4")
                .set_duration(info.get_duration())
                .set_codecs(video_avc.to_string())
                .set_bandwidth(bitrate)
                .set_args([("height", video_stream.height.clone().unwrap())])
                .set_is_default(!should_stream_default)
                .set_target_duration(10)
                .set_label(label);

        stream_tracking.insert(&gid, virtual_manifest).await;
    }

    Ok(should_stream_default)
}

pub async fn create_video(
    info: &FFPStream,
    media: &MediaFile,
    stream_tracking: &StreamTracking,
    gid: &Uuid,
    state: &StateManager,
    prefs: &UserSettings,
    mut should_stream_default: bool,
) -> Result<(), DimErrorWrapper> {
    let video_stream = info
        .get_primary("video")
        .cloned()
        .ok_or(dim_core::errors::StreamingErrors::FileIsCorrupt)?;

    let qualities = get_qualities(
        video_stream.height.unwrap_or(1080) as u64,
        video_stream
            .get_bitrate()
            .or(info.get_container_bitrate())
            .unwrap_or(10_000_000),
    );

    for quality in qualities {
        let bitrate = video_stream
            .get_bitrate()
            .or(Some(quality.bitrate))
            .unwrap()
            .min(quality.bitrate);

        let ctx = ProfileContext {
            file: media.target_file.clone(),
            input_ctx: video_stream.clone().into(),
            output_ctx: OutputCtx {
                codec: "h264".into(),
                start_num: 0,
                bitrate: Some(bitrate),
                height: Some(quality.height as i64),
                ..Default::default()
            },
            ..Default::default()
        };

        let global_prefs = dim_core::settings::get_global_settings();

        let profile_chain = get_profile_for(StreamType::Video, &ctx);
        let profile_chain = if !global_prefs.enable_hwaccel {
            profile_chain
                .into_iter()
                .filter(|x| x.profile_type() != ProfileType::HardwareTranscode)
                .collect::<Vec<_>>()
        } else {
            profile_chain
        };

        debug_assert!(!profile_chain.is_empty());

        let video = state.create(profile_chain, ctx).await?;

        let video_stream_height = video_stream.height.unwrap_or(1080) as u64;
        let ratio = video_stream_height as f64 / quality.height as f64;
        let width = video_stream.width.unwrap_or(1920) as f64 / ratio;

        let video_avc = video_stream
            .level
            .and_then(|x| level_to_tag(x))
            .unwrap_or(get_avc1_tag(
                width as u64,
                quality.height,
                quality.bitrate,
                24,
            ));

        let label = quality_to_label(quality.bitrate, quality.height, Some(bitrate));

        // TODO: This code will not work correctly if there are similar resolutions with different
        // brates.
        let should_be_default = should_stream_default
            || matches!(prefs.default_video_quality, DefaultVideoQuality::Resolution(res, _) if res == quality.height);

        let chunk_path = format!("{}/data/$Number$.m4s", video.clone());
        let init_seg = Some(format!("{}/data/init.mp4", video.clone()));
        let virtual_manifest =
            VirtualManifest::new(video.clone(), chunk_path, init_seg, ContentType::Video)
                .set_mime("video/mp4")
                .set_duration(info.get_duration())
                .set_codecs(video_avc.to_string())
                .set_bandwidth(bitrate)
                .set_args([("height", quality.height)])
                .set_is_default(should_be_default)
                .set_label(label);

        stream_tracking.insert(&gid, virtual_manifest).await;
        // we wan to default only the first stream.
        if should_be_default {
            should_stream_default = false;
        }
    }
    Ok(())
}

pub async fn create_audio(
    info: &FFPStream,
    media: &MediaFile,
    stream_tracking: &StreamTracking,
    gid: &Uuid,
    state: &StateManager,
) -> Result<(), DimErrorWrapper> {
    let audio_streams = info.find_by_type("audio");

    for stream in audio_streams {
        let is_default = info.get_primary("audio") == Some(stream);
        let bitrate = stream
            .bit_rate
            .as_ref()
            .and_then(|x| x.parse::<u64>().ok())
            .unwrap_or(120_000);

        let ctx = ProfileContext {
            file: media.target_file.clone(),
            input_ctx: stream.clone().into(),
            output_ctx: OutputCtx {
                codec: "aac".into(),
                start_num: 0,
                bitrate: Some(bitrate),
                ..Default::default()
            },
            ..Default::default()
        };

        let profile = get_profile_for(StreamType::Audio, &ctx);
        let audio = state.create(profile, ctx).await?;

        let audio_lang = stream
            .get_language()
            .as_deref()
            .and_then(dim_core::utils::lang_from_iso639)
            .unwrap_or("Unknown");

        let audio_codec = dim_core::utils::codec_pretty(stream.get_codec());
        let audio_ch = dim_core::utils::channels_pretty(stream.channels.unwrap_or(2));

        let label = format!("{} ({} {})", audio_lang, audio_codec, audio_ch);

        let chunk_path = format!("{}/data/$Number$.m4s", audio.clone());
        let init_seg = Some(format!("{}/data/init.mp4", audio.clone()));
        let virtual_manifest =
            VirtualManifest::new(audio.clone(), chunk_path, init_seg, ContentType::Audio)
                .set_mime("audio/mp4")
                .set_codecs("mp4a.40.2")
                .set_bandwidth(bitrate)
                .set_is_default(is_default)
                .set_label(label)
                .set_lang(stream.get_language());

        stream_tracking.insert(&gid, virtual_manifest).await;
    }

    Ok(())
}

pub async fn create_subtitles(
    info: &FFPStream,
    media: &MediaFile,
    stream_tracking: &StreamTracking,
    gid: &Uuid,
    state: &StateManager,
    force_ass: bool,
) -> Result<(), DimErrorWrapper> {
    let subtitles = info.find_by_type("subtitle");

    for stream in subtitles {
        let is_default = info.get_primary("subtitle") == Some(stream);
        let is_ssa = ["ssa", "ass"].contains(&stream.codec_name.as_str()) && force_ass;

        if !["subrip", "ass", "ssa", "srt", "webvtt", "vtt"].contains(&stream.codec_name.as_str()) {
            // FIXME: hdmv_pgs_subtitle are not supported yet.
            continue;
        }

        let (mime, codec, output_codec) = if is_ssa {
            ("text/ass", "ass", "ass")
        } else {
            ("text/vtt", "vtt", "webvtt")
        };

        let ctx = ProfileContext {
            file: media.target_file.clone(),
            input_ctx: stream.clone().into(),
            output_ctx: OutputCtx {
                codec: output_codec.into(),
                outdir: "-".into(),
                ..Default::default()
            },
            ..Default::default()
        };

        let lang = stream
            .get_language()
            .as_deref()
            .and_then(dim_core::utils::lang_from_iso639)
            .unwrap_or("Unknown")
            .to_string();

        let title = stream.get_title().unwrap_or(lang.clone());

        let profile_chain = get_profile_for(StreamType::Subtitle, &ctx);
        let subtitle = state.create(profile_chain, ctx).await?;

        let chunk_path = if is_ssa {
            format!("{}/data/stream.ass", subtitle.clone())
        } else {
            format!("{}/data/stream.vtt", subtitle.clone())
        };

        let virtual_manifest =
            VirtualManifest::new(subtitle.clone(), chunk_path, None, ContentType::Subtitle)
                .set_mime(mime)
                .set_codecs(codec)
                .set_bandwidth(1024)
                .set_is_default(is_default)
                .set_label(stream.get_title().unwrap_or(lang.clone()))
                .set_lang(stream.get_language());

        let title = title.replace("&", "and"); // dash.js seems to note like when there are `&` within titles.
        let virtual_manifest = virtual_manifest.set_args([("title".to_string(), title)]);

        stream_tracking.insert(&gid, virtual_manifest).await;
    }

    Ok(())
}

#[derive(Deserialize)]
pub struct ManifestParams {
    start_num: Option<u64>,
    should_kill: Option<bool>,
    includes: Option<String>,
}

/// Method mapped to `/api/v1/stream/<gid>/manifest.mpd` compiles a virtual manifest into a
/// mpeg-dash manifest.
///
/// # Query args
/// * `start_num` - first chunk number
/// * `should_kill` - indicates whether we should clean old streams up while compiling the
/// manifest.
/// * `includes` - ids of streams to include, comma separated.
pub async fn return_manifest(
    State(AppState {
        state,
        stream_tracking,
        ..
    }): State<AppState>,
    Path(gid): Path<String>,
    Query(params): Query<ManifestParams>,
) -> Result<impl IntoResponse, DimErrorWrapper> {
    let gid = match Uuid::parse_str(gid.as_str()) {
        Ok(x) => x,
        Err(_) => return Err(dim_core::errors::StreamingErrors::GidParseError.into()),
    };
    if params.should_kill.unwrap_or(true) {
        let ids = stream_tracking
            .get_for_gid(&gid)
            .await
            .into_iter()
            .filter(|x| !matches!(x.content_type, ContentType::Video | ContentType::Audio))
            .map(|x| x.id)
            .collect::<Vec<_>>();
        stream_tracking.kill(&state, &gid, ids, true).await;
    }

    let manifest = if let Some(includes) = params.includes {
        let includes = includes
            .split(",")
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        stream_tracking
            .compile_only(&gid, params.start_num.unwrap_or(0), includes)
            .await
            .unwrap()
    } else {
        stream_tracking
            .compile(&gid, params.start_num.unwrap_or(0))
            .await
            .unwrap()
    };

    Ok(([(header::CONTENT_TYPE, "application/dash+xml")], manifest))
}

/// Repeatedly invoke a nightfall routine until a timeout occurs waiting for a chunk to be "ready".
///
/// `tick_dur` will the the duration amount that gets passed into `std::thread::sleep` and it will
/// block for AT MOST `tick_limit` ticks. When a the tick limit has been hit `None` is returned
/// otherwise `Some(Result<T, NightfallError>)` is returned.
///
async fn timeout_segment<F, T>(
    f: impl Fn() -> F,
    tick_dur: Duration,
    tick_limit: usize,
) -> Result<T, NightfallError>
where
    F: Future<Output = Result<T, NightfallError>>,
{
    let mut ticks = 0usize;

    loop {
        if ticks >= tick_limit {
            return Err(NightfallError::ChunkNotDone.into());
        }

        let result = f().await;

        if let Err(NightfallError::ChunkNotDone) = result {
            ticks += 1;
            tokio::time::sleep(tick_dur).await;
        } else {
            break result;
        }
    }
}

#[derive(Deserialize)]
pub struct InitParams {
    start_num: Option<u32>,
}

/// Method mapped to `/api/v1/stream/<id>/data/init.mp4` returns the init chunk of the stream `id`.
///
/// # Query args
/// * `start_num` - first chunk index
pub async fn get_init(
    State(AppState { state, .. }): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<InitParams>,
) -> Result<impl IntoResponse, DimErrorWrapper> {
    let path: String = timeout_segment(
        || state.chunk_init_request(id.clone(), params.start_num.unwrap_or(0)),
        Duration::from_millis(100),
        100,
    )
    .await?;

    Ok(reply_with_file(path, ("Content-Type", "video/mp4")).await)
}

/// Method mapped to `/api/v1/stream/<id>/data/<chunk..>` returns a chunk for stream `id`.
pub async fn get_chunk(
    State(AppState { state, .. }): State<AppState>,
    Path((id, chunk)): Path<(String, PathBuf)>,
) -> Result<impl IntoResponse, DimErrorWrapper> {
    let extension = chunk
        .extension()
        .ok_or(dim_core::errors::StreamingErrors::InvalidRequest)?
        .to_string_lossy()
        .into_owned();

    // Chunks will always be m4s or mp4
    if extension.as_str() != "m4s" {
        return Err(dim_core::errors::StreamingErrors::InvalidRequest.into());
    }

    // Parse the chunk filename into a u64, we unwrap_or because sometimes it can be a init chunk,
    // if its a init chunk we assume a chunk index of 0 because we are fetching the first few
    // chunks.
    let chunk_num = chunk
        .file_stem()
        .ok_or(dim_core::errors::StreamingErrors::InvalidRequest)?
        .to_string_lossy()
        .into_owned()
        .parse::<u32>()
        .unwrap_or(0);

    let path: String = timeout_segment(
        || state.chunk_request(id.clone(), chunk_num),
        Duration::from_millis(100),
        100,
    )
    .await?;

    Ok(reply_with_file(path, ("Content-Type", "video/mp4")).await)
}

/// Method mapped to `/api/v1/stream/<id>/data/stream.vtt` attempts to transcode the underlying
/// stream to VTT.
///
/// # Arguments
/// * `id` - id of the underlying stream (Must be a subtitle stream of non-bitmap format).
pub async fn get_subtitle(
    State(AppState { state, .. }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, DimErrorWrapper> {
    let path: String = timeout_segment(
        || state.get_sub(id.clone(), "stream".into()),
        Duration::from_millis(100),
        200,
    )
    .await?;

    Ok(reply_with_file(path, ("Content-Type", "text/vtt")).await)
}

/// Method mapped to `/api/v1/stream/<id>/data/stream.ass` attempts to transcode the underlying
/// stream to ASS.
///
/// # Arguments
/// * `id` - id of the underlying stream (Must be a subtitle stream of non-bitmap format).
pub async fn get_subtitle_ass(
    State(AppState { state, .. }): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, DimErrorWrapper> {
    let path: String = timeout_segment(
        || async {
            if state.has_started(id.clone()).await.unwrap_or(false) {
                if state.is_done(id.clone()).await.unwrap_or(false) {
                    return state.get_sub(id.clone(), "stream".into()).await;
                }
            } else {
                let _ = state.start(id.clone()).await;
            }

            Err(NightfallError::ChunkNotDone)
        },
        Duration::from_millis(100),
        200,
    )
    .await?;

    Ok(reply_with_file(path, ("Content-Type", "text/ass")).await)
}

/// Method mapped to `/api/v1/stream/<gid>/state/should_hard_seek/<chunk_num>` returns whether the
/// client should hard seek in order to play the video at `chunk_num`. This is really only useful
/// on web platforms.
pub async fn should_client_hard_seek(
    State(AppState {
        state,
        stream_tracking,
        ..
    }): State<AppState>,
    Path((gid, chunk_num)): Path<(String, u32)>,
) -> Result<impl IntoResponse, DimErrorWrapper> {
    let gid = match Uuid::parse_str(gid.as_str()) {
        Ok(x) => x,
        Err(_) => return Err(dim_core::errors::StreamingErrors::GidParseError.into()),
    };
    let ids = stream_tracking.get_for_gid(&gid).await;

    let mut should_client_hard_seek = false;

    for manifest in ids {
        should_client_hard_seek |= state.should_hard_seek(manifest.id, chunk_num).await?;
    }

    Ok(Json(&json!({
        "should_client_seek": should_client_hard_seek,
    }))
    .into_response())
}

/// Method mapped to `/api/v1/stream/<gid>/state/get_stderr` attempts to fetch and return the
/// stderr logs of all ffmpeg streams for `gid`.
pub async fn session_get_stderr(
    State(AppState {
        state,
        stream_tracking,
        ..
    }): State<AppState>,
    Path(gid): Path<String>,
) -> Result<impl IntoResponse, DimErrorWrapper> {
    let gid = match Uuid::parse_str(gid.as_str()) {
        Ok(x) => x,
        Err(_) => return Err(dim_core::errors::StreamingErrors::GidParseError.into()),
    };
    Ok(Json(&json!({
    "errors": stream::iter(stream_tracking
        .get_for_gid(&gid)
        .await)
        .filter_map(|x| async { state.get_stderr(x.id).await.ok() })
        .collect::<Vec<_>>().await,
    }))
    .into_response())
}

/// Method mapped to `/api/v1/stream/<gid>/state/kill` will kill all streams for `gid`.
pub async fn kill_session(
    State(AppState {
        state,
        stream_tracking,
        ..
    }): State<AppState>,
    Path(gid): Path<String>,
) -> Result<impl IntoResponse, DimErrorWrapper> {
    let gid = match Uuid::parse_str(gid.as_str()) {
        Ok(x) => x,
        Err(_) => return Err(dim_core::errors::StreamingErrors::GidParseError.into()),
    };
    for manifest in stream_tracking.get_for_gid(&gid).await {
        let _ = state.die(manifest.id).await;
    }

    Ok(StatusCode::NO_CONTENT)
}

use axum::body::Body;
use tokio::io::AsyncReadExt;

async fn reply_with_file(file: String, header: (&str, &str)) -> Response<Body> {
    if let Ok(mut file) = File::open(file).await {
        // FIXME: Super ugly temporary solution (might be slow)
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).await;

        Response::builder()
            .header(header.0, header.1)
            .status(StatusCode::OK)
            .body(Body::from(buf))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
