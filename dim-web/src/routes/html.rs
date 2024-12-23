use crate::error::DimHtmlErrorWrapper;
use crate::middleware::get_cookie_token_value;
use crate::routes::auth;
use crate::AppState;
use askama::Template;
use axum::body;
use axum::body::Body;
use axum::body::Empty;
use axum::extract::Form;
use axum::extract::Path;
use axum::extract::State;
use axum::http::Request;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::response::Response;
use axum::Extension;
use axum_flash::Flash;
use axum_flash::IncomingFlashes;
use dim_core::errors::DimError;
use dim_core::errors::StreamingErrors;
use dim_core::streaming::ffprobe::FFProbeCtx;
use dim_database::asset::Asset;
use dim_database::library::Library;
use dim_database::library::MediaType;
use dim_database::mediafile::MediaFile;
use dim_database::user::verify;
use dim_database::user::InsertableUser;
use dim_database::user::Login;
use dim_database::user::User;
use http::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use std::path;

#[derive(sqlx::FromRow)]
pub struct RecentMedia {
    /// unique id.
    pub id: i64,
    /// id of the library that this media objects belongs to.
    pub library_id: i64,
    /// name of the TV show
    pub name: String,
    /// name of the episode
    pub episode_name: String,
    /// Year in which this tv show was released/aired.
    pub year: i64,
    /// Date when this media object was created and inserted into the database. Used by several
    /// routes to return sorted lists of medias, based on when they were scanned and inserted into
    /// the db.
    pub added: Option<String>,
    /// Path to the media poster.
    pub poster_path: String,
    /// Path to the backdrop for this media object.
    pub backdrop_path: String,
    /// Season number of episode
    pub season: i64,
    /// Episode number of episode
    pub episode: i64,
    /// amount of time into media that has been watched
    pub progress: i64,
    /// duration of media
    pub duration: i64,
    /// mediafile id
    pub file_id: i64,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    username: String,
    avatar: String,
    libraries: Vec<Library>,
    media_by_library: HashMap<i64, Vec<RecentMedia>>,
}

pub async fn index(
    Extension(user): Extension<User>,
    State(AppState { conn, .. }): State<AppState>,
) -> Result<impl IntoResponse, DimHtmlErrorWrapper> {
    let mut tx = conn.read().begin().await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    let libraries = Library::get_all(&mut tx).await;
    let avatar = match Asset::get_of_user(&mut tx, user.id)
        .await
        .ok()
        .map(|x| format!("/images/{}", x.local_path))
    {
        Some(res) => res,
        None => "".to_string(),
    };
    let mut media_by_library = HashMap::<i64, Vec<RecentMedia>>::new();
    for library in &libraries {
        match library.media_type {
            MediaType::Movie => {
                let media = sqlx::query_as::<_, RecentMedia>(
                    r#"
                        SELECT
                            media.id,
                            media.library_id,
                            media.name,
                            "" AS episode_name,
                            media.year AS "year",
                            media.added,
                            media.poster_path AS "poster_path",
                            media.backdrop_path,
                            0 AS "season",
                            0 AS "episode",
                            COALESCE(progress.delta, 0) AS "progress",
                            mediafile.duration AS "duration",
                            mediafile.id AS "file_id"
                        FROM media
                            JOIN library ON library.id = media.library_id
                            INNER JOIN mediafile ON mediafile.media_id = media.id
                            LEFT JOIN progress ON (progress.media_id = media.id AND progress.user_id = ?)
                        WHERE library.id = ?
                        ORDER BY media.added DESC
                        LIMIT 10;
                    "#
                )
                .bind(user.id)
                .bind(library.id)
                .fetch_all(&mut tx)
                .await
                .map_err(|error| {
                    DimHtmlErrorWrapper(DimError::DatabaseError {
                        description: error.to_string(),
                    })
                })?;
                media_by_library.insert(library.id, media);
            }
            MediaType::Tv => {
                let media = sqlx::query_as::<_, RecentMedia>(
                    r#"
                        SELECT
                            media.id,
                            media.library_id,
                            media_show.name,
                            media.name AS episode_name,
                            media_show.year AS "year",
                            media.added,
                            media_show.poster_path AS "poster_path",
                            media.backdrop_path,
                            mediafile.season AS "season",
                            mediafile.episode AS "episode",
                            COALESCE(progress.delta, 0) AS "progress",
                            mediafile.duration AS "duration",
                            mediafile.id AS "file_id"
                        FROM media
                            JOIN library ON library.id = media.library_id
                            INNER JOIN mediafile ON mediafile.media_id = media.id
                            JOIN episode ON episode.id = media.id
                            JOIN season ON season.id = episode.seasonid
                            JOIN media media_show ON media_show.id = season.tvshowid
                            LEFT JOIN progress ON (progress.media_id = media.id AND progress.user_id = ?)
                        WHERE library.id = ?
                        ORDER BY media.added DESC
                        LIMIT 10;
                    "#
                )
                .bind(user.id)
                .bind(library.id)
                .fetch_all(&mut tx)
                .await
                .map_err(|error| {
                    DimHtmlErrorWrapper(DimError::DatabaseError {
                        description: error.to_string(),
                    })
                })?;
                media_by_library.insert(library.id, media);
            }
            _ => {}
        };
    }
    Ok(IndexTemplate {
        username: user.username,
        avatar,
        libraries,
        media_by_library,
    })
}

#[derive(Template)]
#[template(path = "play.html")]
pub struct PlayTemplate {
    id: i64,
    media_configurations: String,
}

pub async fn play(
    State(AppState { conn, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, DimHtmlErrorWrapper> {
    let mut tx = conn.read().begin().await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    let media = MediaFile::get_one(&mut tx, id).await.map_err(|e| {
        DimHtmlErrorWrapper(DimError::StreamingError(StreamingErrors::NoMediaFileFound(
            e.to_string(),
        )))
    })?;

    let target_file = media.target_file.clone();

    // FIXME: When `fs::try_exists` gets stabilized we should use that as it will allow us to
    // detect if the user lacks permissions to access the file, etc.
    if !path::Path::new(&target_file).exists() {
        return Err(DimHtmlErrorWrapper(DimError::StreamingError(
            StreamingErrors::FileDoesNotExist,
        )));
    }

    let info = FFProbeCtx::new(dim_core::streaming::FFPROBE_BIN.as_ref())
        .get_meta(target_file)
        .await
        .map_err(|_| {
            DimHtmlErrorWrapper(DimError::StreamingError(StreamingErrors::FFProbeCtxFailed))
        })?;

    let mut ms = info
        .get_ms()
        .ok_or(DimHtmlErrorWrapper(DimError::StreamingError(
            StreamingErrors::FileIsCorrupt,
        )))?
        .to_string();

    ms.truncate(4);

    Ok(PlayTemplate {
        id,
        media_configurations: serde_json::to_string(&info.get_media_configurations())
            .expect("failed to serialize Vec of MediaConfiguration"),
    })
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    flashes: Vec<(String, String)>,
}

pub async fn login(
    State(AppState { conn, .. }): State<AppState>,
    flashes: IncomingFlashes,
    request: Request<Body>,
) -> (IncomingFlashes, impl IntoResponse) {
    let mut flashes_for_template = Vec::new();
    for (level, text) in flashes.clone().iter() {
        let level_string = format!("{:?}", level);
        flashes_for_template.push((level_string, text.to_owned()));
    }
    match get_cookie_token_value(&request) {
        Some(_) => {
            // If there is a token cookie, redirect to dashboard
            return (flashes, Redirect::to("/").into_response());
        }
        _ => {}
    }
    if auth::is_admin_exists(conn).await.unwrap_or(false) {
        (
            flashes,
            Html(
                LoginTemplate {
                    flashes: flashes_for_template,
                }
                .render()
                .unwrap(),
            )
            .into_response(),
        )
    } else {
        (flashes, Redirect::to("/register").into_response())
    }
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn handle_login(
    State(AppState { conn, .. }): State<AppState>,
    flash: Flash,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse, DimHtmlErrorWrapper> {
    let mut tx = conn.read().begin().await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    let user = User::get(&mut tx, &form.username).await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    let pass = user.get_pass(&mut tx).await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    if verify(user.username, pass, form.password) {
        let token = Login::create_cookie(user.id);

        return Ok(Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", "/")
            // Set token cookie max age to 1 year
            .header(
                "Set-Cookie",
                format!(
                    "token={}; Path=/; Max-Age={}; SameSite=Strict; HttpOnly",
                    token, 31536000
                ),
            )
            .body(body::boxed(Empty::new()))
            .unwrap());
    }

    let message = flash.error("The provided username or password is incorrect.");
    Ok((message, Redirect::to("/login").into_response()).into_response())
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    admin_exists: bool,
    flashes: Vec<(String, String)>,
}

pub async fn register(
    State(AppState { conn, .. }): State<AppState>,
    flashes: IncomingFlashes,
) -> impl IntoResponse {
    let mut flashes_for_template = Vec::new();
    for (level, text) in flashes.clone().iter() {
        let level_string = format!("{:?}", level);
        flashes_for_template.push((level_string, text.to_owned()));
    }
    (
        flashes,
        Html(
            RegisterTemplate {
                admin_exists: auth::is_admin_exists(conn).await.unwrap_or(false),
                flashes: flashes_for_template,
            }
            .render()
            .unwrap(),
        )
        .into_response(),
    )
}

pub async fn handle_register(
    State(AppState { conn, .. }): State<AppState>,
    flash: Flash,
    Form(new_user): Form<Login>,
) -> Result<impl IntoResponse, DimHtmlErrorWrapper> {
    let mut lock = conn.writer().lock_owned().await;
    let mut tx = dim_database::write_tx(&mut lock).await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;
    let users_empty = User::get_all(&mut tx)
        .await
        .map_err(|err| {
            DimHtmlErrorWrapper(DimError::DatabaseError {
                description: err.to_string(),
            })
        })?
        .is_empty();

    if !users_empty
        && (new_user.invite_token.is_none()
            || !new_user.invite_token_valid(&mut tx).await.map_err(|err| {
                DimHtmlErrorWrapper(DimError::DatabaseError {
                    description: err.to_string(),
                })
            })?)
    {
        return Ok((
            flash.error(DimError::NoToken.to_string()),
            Redirect::to("/register").into_response(),
        )
            .into_response());
    }

    let roles = dim_database::user::Roles(if !users_empty {
        vec!["user".to_string()]
    } else {
        vec!["owner".to_string()]
    });

    let claimed_invite = if users_empty {
        Login::new_invite(&mut tx).await.map_err(|err| {
            DimHtmlErrorWrapper(DimError::DatabaseError {
                description: err.to_string(),
            })
        })?
    } else {
        match new_user.invite_token {
            Some(token) => token,
            None => {
                return Ok((
                    flash.error(DimError::NoToken.to_string()),
                    Redirect::to("/register").into_response(),
                )
                    .into_response());
            }
        }
    };

    let res = InsertableUser {
        username: new_user.username.clone(),
        password: new_user.password.clone(),
        roles,
        claimed_invite,
        prefs: Default::default(),
    }
    .insert(&mut tx)
    .await
    .map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;

    tx.commit().await.map_err(|err| {
        DimHtmlErrorWrapper(DimError::DatabaseError {
            description: err.to_string(),
        })
    })?;

    Ok((
        flash.info(format!(
            "Please login with your newly created user: {}.",
            res.username
        )),
        Redirect::to("/login").into_response(),
    )
        .into_response())
}

pub async fn handle_logout() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", "/login")
        .header(
            "Set-Cookie",
            "token=TO_BE_DELETED; Path=/; Max-Age=-1; SameSite=Strict; HttpOnly",
        )
        .body(body::boxed(Empty::new()))
        .unwrap()
}
