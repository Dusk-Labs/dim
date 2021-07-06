use auth::Wrapper as Auth;
use errors::StreamingErrors;

use crate::core::DbConnection;
use crate::core::StateManager;
use crate::errors;
use crate::stream_tracking::ContentType;
use crate::stream_tracking::StreamTracking;
use crate::stream_tracking::VirtualManifest;
use crate::streaming::ffprobe::FFProbeCtx;
use crate::streaming::get_avc1_tag;
use crate::streaming::level_to_tag;
use crate::streaming::Avc1Level;

use chrono::prelude::*;
use chrono::NaiveDateTime;
use chrono::Utc;

use database::mediafile::MediaFile;

use slog::info;
use slog::Logger;

use nightfall::error::NightfallError;
use nightfall::profiles::*;

use std::collections::HashMap;
use std::future::Future;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use futures::stream;
use futures::StreamExt;

use tokio::fs::File;
use tokio::task::spawn_blocking;

use serde_json::json;

use uuid::Uuid;
use warp::http::status::StatusCode;
use warp::reply;
use warp::Filter;

pub fn stream_router(
    conn: DbConnection,
    state: StateManager,
    stream_tracking: StreamTracking,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::return_virtual_manifest(conn.clone(), state.clone(), stream_tracking.clone())
        .or(filters::return_manifest(
            conn.clone(),
            state.clone(),
            stream_tracking.clone(),
        ))
        .or(filters::get_init(state.clone()))
        .or(filters::should_client_hard_seek(
            state.clone(),
            stream_tracking.clone(),
        ))
        .or(filters::session_get_stderr(
            state.clone(),
            stream_tracking.clone(),
        ))
        .or(filters::kill_session(
            state.clone(),
            stream_tracking.clone(),
        ))
        .or(filters::get_subtitle(state.clone()))
        .or(filters::get_chunk(state.clone()))
        .recover(super::global_filters::handle_rejection)
}

mod filters {
    use warp::reject;
    use warp::Filter;

    use crate::core::DbConnection;
    use crate::core::StateManager;
    use crate::errors::StreamingErrors;
    use crate::stream_tracking::StreamTracking;

    use auth::Wrapper as Auth;
    use uuid::Uuid;

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
        }

        warp::path!("api" / "v1" / "stream" / i64 / "manifest")
            .and(warp::get())
            .and(warp::query::query::<QueryArgs>())
            .and(auth::with_auth())
            .and(with_state::<DbConnection>(conn))
            .and(with_state::<StateManager>(state))
            .and(with_state::<StreamTracking>(stream_tracking))
            .and_then(
                |id: i64,
                 QueryArgs { gid }: QueryArgs,
                 auth: Auth,
                 conn: DbConnection,
                 state: StateManager,
                 stream_tracking: StreamTracking| async move {
                    let gid = gid.and_then(|x| Uuid::parse_str(x.as_str()).ok());
                    super::return_virtual_manifest(state, stream_tracking, auth, conn, id, gid)
                        .await
                        .map_err(|e| reject::custom(e))
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
            .and(auth::with_auth())
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
                 auth: Auth,
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
                    .map_err(|e| reject::custom(e))
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
    auth: Auth,
    conn: DbConnection,
    id: i64,
    gid: Option<Uuid>,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
    if let Some(gid) = gid {
        return Ok(reply::json(&json!({
            "tracks": stream_tracking.get_for_gid(&gid).await,
            "gid": gid.to_hyphenated().to_string(),
        })));
    }

    let gid = uuid::Uuid::new_v4();

    let media = MediaFile::get_one(&conn, id)
        .await
        .map_err(|e| errors::StreamingErrors::NoMediaFileFound(e.to_string()))?;

    let user_id = auth.0.claims.id;

    let target_file = media.target_file.clone();
    let info = spawn_blocking(move || {
        FFProbeCtx::new(crate::streaming::FFPROBE_BIN.as_ref())
            .get_meta(&std::path::PathBuf::from(target_file))
    })
    .await
    .unwrap()
    .map_err(|_| errors::StreamingErrors::FFProbeCtxFailed)?;

    let mut ms = info
        .get_ms()
        .ok_or(errors::StreamingErrors::FileIsCorrupt)?
        .to_string();

    ms.truncate(4);

    let duration = chrono::DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(info.get_duration().unwrap() as i64, 0),
        Utc,
    );

    let video_stream = info
        .find_by_codec("video")
        .first()
        .cloned()
        .ok_or(errors::StreamingErrors::FileIsCorrupt)?;

    let base_ctx = ProfileContext {
        start_num: 0,
        file: media.target_file.clone(),
        ..Default::default()
    };

    let profile = get_profile_for(StreamType::Video, &video_stream.codec_name, "h264")
        .pop()
        .expect("Failed to find a supported transcoding profile.");

    let video = state
        .create(
            profile,
            ProfileContext {
                stream: video_stream.index as usize,
                ..base_ctx.clone()
            },
        )
        .await?;

    // FIXME: Stop hardcoding a fps of 24
    let video_avc = video_stream
        .level
        .and_then(|x| level_to_tag(x))
        .unwrap_or(get_avc1_tag(
            video_stream.width.clone().unwrap_or(1920) as u64,
            video_stream.height.clone().unwrap_or(1080) as u64,
            info.get_bitrate().parse().unwrap(),
            24,
        ));

    stream_tracking
        .insert(
            &gid,
            VirtualManifest {
                id: video.clone(),
                is_direct: true,
                mime: "video/mp4".into(),
                duration: info.get_duration(),
                content_type: ContentType::Video,
                chunk_path: format!("{}/data/$Number$.m4s", video.clone()),
                init_seg: Some(format!("{}/data/init.mp4", video.clone())),
                codecs: video_avc.to_string(),
                bandwidth: info.get_bitrate().parse::<u64>().unwrap(),
                args: {
                    let mut x = HashMap::new();
                    x.insert(
                        "height".to_string(),
                        video_stream.height.clone().unwrap().to_string(),
                    );
                    x
                },
            },
        )
        .await;

    let audio_streams = info.find_by_codec("audio");

    for stream in audio_streams {
        let profile = get_profile_for(StreamType::Audio, &stream.codec_name, "aac")
            .pop()
            .expect("Failed to find a supported transcoding profile.");
        let audio = state
            .create(
                profile,
                ProfileContext {
                    stream: stream.index as usize,
                    ..base_ctx.clone()
                },
            )
            .await?;

        stream_tracking
            .insert(
                &gid,
                VirtualManifest {
                    id: audio.clone(),
                    is_direct: false,
                    mime: "audio/mp4".into(),
                    duration: info.get_duration(),
                    codecs: "mp4a.40.2".into(),
                    bandwidth: 120_000,
                    content_type: ContentType::Audio,
                    chunk_path: format!("{}/data/$Number$.m4s", audio.clone()),
                    init_seg: Some(format!("{}/data/init.mp4", audio.clone())),
                    args: HashMap::new(),
                },
            )
            .await;
    }

    let subtitles = info.find_by_codec("subtitle");

    for stream in subtitles {
        match get_profile_for(StreamType::Subtitle, &stream.codec_name, "webvtt").pop() {
            Some(profile) => {
                let subtitle = state
                    .create(
                        profile,
                        ProfileContext {
                            stream: stream.index as usize,
                            outdir: "-".into(),
                            ..base_ctx.clone()
                        },
                    )
                    .await?;

                stream_tracking
                    .insert(
                        &gid,
                        VirtualManifest {
                            id: subtitle.clone(),
                            is_direct: false,
                            content_type: ContentType::Subtitle,
                            mime: "text/vtt".into(),
                            codecs: "vtt".into(), //ignored
                            bandwidth: 0,         // ignored
                            duration: None,
                            chunk_path: format!("{}/data/stream.vtt", subtitle.clone()),
                            init_seg: None,
                            args: {
                                let mut x = HashMap::new();
                                if let Some(y) = stream
                                    .tags
                                    .as_ref()
                                    .and_then(|x| x.title.clone().or(x.language.clone()))
                                {
                                    x.insert("title".to_string(), y);
                                }
                                x
                            },
                        },
                    )
                    .await;
            }
            None => {}
        }
    }

    Ok(reply::json(&json!({
        "tracks": stream_tracking.get_for_gid(&gid).await,
        "gid": gid.to_hyphenated().to_string(),
    })))
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
    auth: Auth,
    conn: DbConnection,
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
//#[get("/<id>/data/init.mp4?<start_num>", rank = 1)]
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
// #[get("/<id>/data/<chunk..>", rank = 3)]
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
//#[get("/<id>/data/stream.vtt", rank = 2)]
pub async fn get_subtitle(
    state: StateManager,
    id: String,
) -> Result<impl warp::Reply, errors::StreamingErrors> {
    let path: String = timeout_segment(
        || state.get_sub(id.clone(), "stream.vtt".into()),
        Duration::from_millis(100),
        200,
    )
    .await?;

    Ok(reply_with_file(path, ("Content-Type", "text/vtt")).await)
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
        file.read_to_end(&mut buf).await;

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
