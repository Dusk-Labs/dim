use crate::core::DbConnection;
use crate::core::StateManager;
use crate::errors;
use crate::stream_tracking::ContentType;
use crate::stream_tracking::StreamTracking;
use crate::stream_tracking::VirtualManifest;
use crate::streaming::ffprobe::FFPStream;
use crate::streaming::ffprobe::FFProbeCtx;
use crate::streaming::get_avc1_tag;
use crate::streaming::get_qualities;
use crate::streaming::level_to_tag;
use crate::utils::quality_to_label;

use dim_database::mediafile::MediaFile;
use dim_database::user::DefaultVideoQuality;
use dim_database::user::User;
use dim_database::user::UserSettings;

use nightfall::error::NightfallError;
use nightfall::profiles::*;

use std::future::Future;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use futures::stream;
use futures::StreamExt;

use tokio::fs::File;

use serde_json::json;

use uuid::Uuid;
use warp::http::status::StatusCode;
use warp::reply;

pub mod filters {
    use warp::reject;
    use warp::reply::Reply;
    use warp::Filter;

    use crate::core::DbConnection;
    use crate::core::StateManager;
    use crate::errors::StreamingErrors;
    use crate::stream_tracking::StreamTracking;
    use crate::warp_unwrap;

    use dim_database::user::User;
    use uuid::Uuid;

    use super::super::global_filters::with_auth;
    use super::super::global_filters::with_state;
    use serde::Deserialize;

    pub fn return_virtual_manifest(
        conn: DbConnection,
        state: StateManager,
        stream_tracking: StreamTracking,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct QueryArgs {
            gid: Option<String>,
            #[serde(default)]
            force_ass: bool,
        }

        warp::path!("api" / "v1" / "stream" / i64 / "manifest")
            .and(warp::get())
            .and(warp::query::query::<QueryArgs>())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and(with_state::<StateManager>(state))
            .and(with_state::<StreamTracking>(stream_tracking))
            .and_then(
                |id: i64,
                 QueryArgs { gid, force_ass }: QueryArgs,
                 auth: User,
                 conn: DbConnection,
                 state: StateManager,
                 stream_tracking: StreamTracking| async move {
                    let gid = gid.and_then(|x| Uuid::parse_str(x.as_str()).ok());

                    warp_unwrap!(
                        super::return_virtual_manifest(
                            state,
                            stream_tracking,
                            auth,
                            conn,
                            id,
                            gid,
                            force_ass
                        )
                        .await
                    )
                },
            )
    }

    pub fn return_manifest(
        conn: DbConnection,
        state: StateManager,
        stream_tracking: StreamTracking,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct QueryArgs {
            start_num: Option<u64>,
            should_kill: Option<bool>,
            includes: Option<String>,
        }

        warp::path!("api" / "v1" / "stream" / String / "manifest.mpd")
            .and(warp::get())
            .and(warp::query::query::<QueryArgs>())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and(with_state::<StateManager>(state))
            .and(with_state::<StreamTracking>(stream_tracking))
            .and_then(
                |id: String,
                 QueryArgs {
                     start_num,
                     should_kill,
                     includes,
                 }: QueryArgs,
                 auth: User,
                 conn: DbConnection,
                 state: StateManager,
                 stream_tracking: StreamTracking| async move {
                    let gid = match Uuid::parse_str(id.as_str()) {
                        Ok(x) => x,
                        Err(_) => return Err(reject::custom(StreamingErrors::GidParseError)),
                    };

                    super::return_manifest(
                        state,
                        stream_tracking,
                        auth,
                        conn,
                        gid,
                        start_num,
                        should_kill,
                        includes,
                    )
                    .await
                    .map_or_else(|x| Ok(x.into_response()), |e| Ok(e.into_response()))
                },
            )
    }

    pub fn get_init(
        state: StateManager,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        #[derive(Deserialize)]
        struct QueryArgs {
            start_num: Option<u32>,
        }

        warp::path!("api" / "v1" / "stream" / String / "data" / "init.mp4")
            .and(warp::get())
            .and(warp::query::query::<QueryArgs>())
            .and(with_state::<StateManager>(state))
            .and_then(
                |id: String, QueryArgs { start_num }: QueryArgs, state: StateManager| async move {
                    super::get_init(state, id, start_num)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn get_chunk(
        state: StateManager,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "stream" / String / "data" / ..)
            .and(warp::get())
            .and(warp::filters::path::tail())
            .and(with_state::<StateManager>(state))
            .and_then(
                |id: String, chunk: warp::filters::path::Tail, state: StateManager| async move {
                    super::get_chunk(state, id, chunk.as_str().into())
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn get_subtitle(
        state: StateManager,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "stream" / String / "data" / "stream.vtt")
            .and(warp::get())
            .and(with_state::<StateManager>(state))
            .and_then(|id: String, state: StateManager| async move {
                super::get_subtitle(state, id)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn get_subtitle_ass(
        state: StateManager,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "stream" / String / "data" / "stream.ass")
            .and(warp::get())
            .and(with_state::<StateManager>(state))
            .and_then(|id: String, state: StateManager| async move {
                super::get_subtitle_ass(state, id)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn should_client_hard_seek(
        state: StateManager,
        stream_tracking: StreamTracking,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "stream" / String / "state" / "should_hard_seek" / u32)
            .and(warp::get())
            .and(with_state(state))
            .and(with_state(stream_tracking))
            .and_then(
                |id: String,
                 chunk_num: u32,
                 state: StateManager,
                 stream_tracking: StreamTracking| async move {
                    let gid = match Uuid::parse_str(id.as_str()) {
                        Ok(x) => x,
                        Err(_) => return Err(reject::custom(StreamingErrors::GidParseError)),
                    };
                    super::should_client_hard_seek(state, stream_tracking, gid, chunk_num)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn session_get_stderr(
        state: StateManager,
        stream_tracking: StreamTracking,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "stream" / String / "state" / "get_stderr")
            .and(warp::get())
            .and(with_state(state))
            .and(with_state(stream_tracking))
            .and_then(
                |id: String, state: StateManager, stream_tracking: StreamTracking| async move {
                    let gid = match Uuid::parse_str(id.as_str()) {
                        Ok(x) => x,
                        Err(_) => return Err(reject::custom(StreamingErrors::GidParseError)),
                    };

                    super::session_get_stderr(state, stream_tracking, gid)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }

    pub fn kill_session(
        state: StateManager,
        stream_tracking: StreamTracking,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "stream" / String / "state" / "kill")
            .and(warp::get())
            .and(with_state(state))
            .and(with_state(stream_tracking))
            .and_then(
                |id: String, state: StateManager, stream_tracking: StreamTracking| async move {
                    let gid = match Uuid::parse_str(id.as_str()) {
                        Ok(x) => x,
                        Err(_) => return Err(reject::custom(StreamingErrors::GidParseError)),
                    };

                    super::kill_session(state, stream_tracking, gid)
                        .await
                        .map_err(|e| reject::custom(e))
                },
            )
    }
}

/// Method mapped to `GET /api/v1/stream/<id>/manifest?<gid>` returns or creates a virtual
/// manifest.
pub async fn return_virtual_manifest(
    state: StateManager,
    stream_tracking: StreamTracking,
    auth: User,
    conn: DbConnection,
    id: i64,
    gid: Option<Uuid>,
    force_ass: bool,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
    if let Some(gid) = gid {
        return Ok(reply::json(&json!({
            "tracks": stream_tracking.get_for_gid(&gid).await,
            "gid": gid.as_hyphenated().to_string(),
        })));
    }

    let mut tx = conn.read().begin().await?;
    let user_prefs = auth.prefs;

    let gid = uuid::Uuid::new_v4();

    let media = MediaFile::get_one(&mut tx, id)
        .await
        .map_err(|e| errors::StreamingErrors::NoMediaFileFound(e.to_string()))?;

    let target_file = media.target_file.clone();

    // FIXME: When `fs::try_exists` gets stabilized we should use that as it will allow us to
    // detect if the user lacks permissions to access the file, etc.
    if !Path::new(&target_file).exists() {
        return Err(errors::StreamingErrors::FileDoesNotExist);
    }

    let info = FFProbeCtx::new(crate::streaming::FFPROBE_BIN.as_ref())
        .get_meta(target_file)
        .await
        .map_err(|_| errors::StreamingErrors::FFProbeCtxFailed)?;

    let mut ms = info
        .get_ms()
        .ok_or(errors::StreamingErrors::FileIsCorrupt)?
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
    create_subtitles(&info, &media, &stream_tracking, &gid, &state, force_ass).await?;

    stream_tracking.generate_sids(&gid).await;

    Ok(reply::json(&json!({
        "tracks": stream_tracking.get_for_gid(&gid).await,
        "gid": gid.as_hyphenated().to_string(),
    })))
}

pub async fn try_create_dstream(
    info: &FFPStream,
    media: &MediaFile,
    stream_tracking: &StreamTracking,
    gid: &Uuid,
    state: &StateManager,
    prefs: &UserSettings,
) -> Result<bool, errors::StreamingErrors> {
    let video_stream = info
        .get_primary("video")
        .cloned()
        .ok_or(errors::StreamingErrors::FileIsCorrupt)?;

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
) -> Result<(), errors::StreamingErrors> {
    let video_stream = info
        .get_primary("video")
        .cloned()
        .ok_or(errors::StreamingErrors::FileIsCorrupt)?;

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

        let global_prefs = super::settings::get_global_settings();

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

        let label = quality_to_label(quality, Some(bitrate));

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
) -> Result<(), errors::StreamingErrors> {
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
            .and_then(crate::utils::lang_from_iso639)
            .unwrap_or("Unknown");

        let audio_codec = crate::utils::codec_pretty(stream.get_codec());
        let audio_ch = crate::utils::channels_pretty(stream.channels.unwrap_or(2));

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
) -> Result<(), errors::StreamingErrors> {
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
            .and_then(crate::utils::lang_from_iso639)
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

/// Method mapped to `/api/v1/stream/<gid>/manifest.mpd` compiles a virtual manifest into a
/// mpeg-dash manifest.
///
/// # Query args
/// * `start_num` - first chunk number
/// * `should_kill` - indicates whether we should clean old streams up while compiling the
/// manifest.
/// * `includes` - ids of streams to include, comma separated.
pub async fn return_manifest(
    state: StateManager,
    stream_tracking: StreamTracking,
    _auth: User,
    _conn: DbConnection,
    gid: Uuid,
    start_num: Option<u64>,
    should_kill: Option<bool>,
    includes: Option<String>,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
    if should_kill.unwrap_or(true) {
        let ids = stream_tracking
            .get_for_gid(&gid)
            .await
            .into_iter()
            .filter(|x| !matches!(x.content_type, ContentType::Video | ContentType::Audio))
            .map(|x| x.id)
            .collect::<Vec<_>>();
        stream_tracking.kill(&state, &gid, ids, true).await;
    }

    let manifest = if let Some(includes) = includes {
        let includes = includes
            .split(",")
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        stream_tracking
            .compile_only(&gid, start_num.unwrap_or(0), includes)
            .await
            .unwrap()
    } else {
        stream_tracking
            .compile(&gid, start_num.unwrap_or(0))
            .await
            .unwrap()
    };

    Ok(warp::reply::with_header(
        manifest,
        "Content-Type",
        "application/dash+xml",
    ))
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

/// Method mapped to `/api/v1/stream/<id>/data/init.mp4` returns the init chunk of the stream `id`.
///
/// # Query args
/// * `start_num` - first chunk index
pub async fn get_init(
    state: StateManager,
    id: String,
    start_num: Option<u32>,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
    let path: String = timeout_segment(
        || state.chunk_init_request(id.clone(), start_num.unwrap_or(0)),
        Duration::from_millis(100),
        100,
    )
    .await?;

    Ok(reply_with_file(path, ("Content-Type", "video/mp4")).await)
}

/// Method mapped to `/api/v1/stream/<id>/data/<chunk..>` returns a chunk for stream `id`.
pub async fn get_chunk(
    state: StateManager,
    id: String,
    chunk: PathBuf,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
    let extension = chunk
        .extension()
        .ok_or(errors::StreamingErrors::InvalidRequest)?
        .to_string_lossy()
        .into_owned();

    // Chunks will always be m4s or mp4
    if extension.as_str() != "m4s" {
        return Err(errors::StreamingErrors::InvalidRequest);
    }

    // Parse the chunk filename into a u64, we unwrap_or because sometimes it can be a init chunk,
    // if its a init chunk we assume a chunk index of 0 because we are fetching the first few
    // chunks.
    let chunk_num = chunk
        .file_stem()
        .ok_or(errors::StreamingErrors::InvalidRequest)?
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
    state: StateManager,
    id: String,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
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
    state: StateManager,
    id: String,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
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
    state: StateManager,
    stream_tracking: StreamTracking,
    gid: Uuid,
    chunk_num: u32,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
    let ids = stream_tracking.get_for_gid(&gid).await;

    let mut should_client_hard_seek = false;

    for manifest in ids {
        should_client_hard_seek |= state.should_hard_seek(manifest.id, chunk_num).await?;
    }

    Ok(reply::json(&json!({
        "should_client_seek": should_client_hard_seek,
    })))
}

/// Method mapped to `/api/v1/stream/<gid>/state/get_stderr` attempts to fetch and return the
/// stderr logs of all ffmpeg streams for `gid`.
pub async fn session_get_stderr(
    state: StateManager,
    stream_tracking: StreamTracking,
    gid: Uuid,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
    Ok(reply::json(&json!({
    "errors": stream::iter(stream_tracking
        .get_for_gid(&gid)
        .await)
        .filter_map(|x| async { state.get_stderr(x.id).await.ok() })
        .collect::<Vec<_>>().await,
    })))
}

/// Method mapped to `/api/v1/stream/<gid>/state/kill` will kill all streams for `gid`.
pub async fn kill_session(
    state: StateManager,
    stream_tracking: StreamTracking,
    gid: Uuid,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
    for manifest in stream_tracking.get_for_gid(&gid).await {
        let _ = state.die(manifest.id).await;
    }

    Ok(StatusCode::NO_CONTENT)
}

use tokio::io::AsyncReadExt;
use warp::http::response::Response;
use warp::hyper::body::Body;

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
