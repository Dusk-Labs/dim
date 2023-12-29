use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::response::Response;
use axum::Extension;

use dim_database::episode::Episode;
use dim_database::genre::Genre;
use dim_database::library::MediaType;
use dim_database::media::Media;
use dim_database::mediafile::MediaFile;
use dim_database::progress::Progress;
use dim_database::user::User;
use dim_database::DatabaseError;

use super::auth::AuthError;

use dim_utils::json;
use serde_json::Value;

pub async fn banners(
    Extension(user): Extension<User>,
    State(AppState { conn, .. }): State<AppState>,
) -> Result<Response, AuthError> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
    let mut banners = Vec::new();
    for media in Media::get_random_with(&mut tx, 10).await? {
        if let Ok(x) = match media.media_type {
            MediaType::Tv => banner_for_show(&mut tx, &user, &media).await,
            MediaType::Movie => banner_for_movie(&mut tx, &user, &media).await,
            _ => unreachable!(),
        } {
            banners.push(x);
        }
    }

    Ok(Json(&banners.iter().take(3).collect::<Vec<_>>()).into_response())
}

async fn banner_for_movie(
    conn: &mut dim_database::Transaction<'_>,
    user: &User,
    media: &Media,
) -> Result<Value, AuthError> {
    let progress = Progress::get_for_media_user(&mut *conn, user.id, media.id)
        .await
        .map(|x| x.delta)
        .unwrap_or(0);

    let mediafiles = MediaFile::get_of_media(&mut *conn, media.id).await?;
    let media_duration = MediaFile::get_largest_duration(&mut *conn, media.id).await?;

    let genres = Genre::get_by_media(&mut *conn, media.id)
        .await
        .map(|x| x.into_iter().map(|x| x.name).collect::<Vec<_>>())
        .unwrap_or_default();

    let caption = if progress > 0 {
        "CONTINUE WATCHING"
    } else {
        "WATCH SOMETHING FRESH"
    };

    Ok(json!({
        "id": media.id,
        "title": media.name,
        "year": media.year,
        "synopsis": media.description,
        "backdrop": media.backdrop_path,
        "duration": media_duration,
        "genres": genres,
        "delta": progress,
        "banner_caption": caption,
        "versions": mediafiles.iter().map(|x| json!({
            "id": x.id,
            "file": x.target_file,
            "display_name": format!("{} - {} - {} - Library {}",
                                    x.codec.as_ref().unwrap_or(&"Unknown VC".to_string()),
                                    x.audio.as_ref().unwrap_or(&"Unknwon AC".to_string()),
                                    x.original_resolution.as_ref().unwrap_or(&"Unknown res".to_string()),
                                    x.library_id)
        })).collect::<Vec<_>>(),
    }))
}

async fn banner_for_show(
    conn: &mut dim_database::Transaction<'_>,
    user: &User,
    media: &Media,
) -> Result<Value, AuthError> {
    let episode = if let Ok(Some(ep)) =
        Episode::get_last_watched_episode(&mut *conn, media.id, user.id).await
    {
        let (delta, duration) = Progress::get_progress_for_media(&mut *conn, ep.id, user.id)
            .await
            .unwrap_or((0, 1));

        if (delta as f64 / duration as f64) > 0.90 {
            ep.get_next_episode(&mut *conn).await.unwrap_or(ep)
        } else {
            ep
        }
    } else {
        Episode::get_first_for_show(&mut *conn, media.id).await?
    };

    let genres = Genre::get_by_media(&mut *conn, media.id)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<_>>();

    let progress = Progress::get_for_media_user(&mut *conn, user.id, episode.id)
        .await
        .map(|x| x.delta)
        .unwrap_or(0);

    let duration = MediaFile::get_largest_duration(&mut *conn, episode.id)
        .await
        .unwrap_or(0);

    let mediafiles = MediaFile::get_of_media(&mut *conn, episode.id).await?;

    let caption = if progress > 0 {
        "CONTINUE WATCHING"
    } else {
        "WATCH SOMETHING FRESH"
    };

    Ok(json!({
        "id": episode.id,
        "title": media.name,
        "year": media.year,
        "synopsis": media.description,
        "backdrop": media.backdrop_path,
        "duration": duration,
        "genres": genres,
        "delta": progress,
        "banner_caption": caption,
        "episode": episode.episode,
        "season": episode.get_season_number(&mut *conn).await.unwrap_or(0),
        "versions": mediafiles.iter().map(|x| json!({
            "id": x.id,
            "file": x.target_file,
            "display_name": format!("{} - {} - {} - Library {}",
                                    x.codec.as_ref().unwrap_or(&"Unknown VC".to_string()),
                                    x.audio.as_ref().unwrap_or(&"Unknwon AC".to_string()),
                                    x.original_resolution.as_ref().unwrap_or(&"Unknown res".to_string()),
                                    x.library_id)
        })).collect::<Vec<_>>(),
    }))
}

pub async fn dashboard(
    Extension(user): Extension<User>,
    State(AppState { conn, .. }): State<AppState>,
) -> Result<Response, AuthError> {
    let mut tx = conn.read().begin().await.map_err(DatabaseError::from)?;
    let mut top_rated = Vec::new();
    for media in Media::get_top_rated(&mut tx, 10).await? {
        let item = match sqlx::query!(
            "SELECT _tblmedia.name, assets.local_path FROM _tblmedia LEFT JOIN assets ON assets.id = _tblmedia.poster
            WHERE _tblmedia.id = ?",
            media
        ).fetch_one(&mut *tx).await {
            Ok(x) => x,
            Err(_) => continue,
        };

        top_rated.push(json!({
            "id": media,
            "poster_path": item.local_path,
            "name": item.name
        }));
    }

    let mut recently_added = Vec::new();
    for media in Media::get_recently_added(&mut tx, 10).await? {
        let item = match sqlx::query!(
            "SELECT _tblmedia.name, assets.local_path FROM _tblmedia LEFT JOIN assets ON assets.id = _tblmedia.poster
            WHERE _tblmedia.id = ?",
            media
        ).fetch_one(&mut *tx).await {
            Ok(x) => x,
            Err(_) => continue,
        };

        recently_added.push(json!({
            "id": media,
            "poster_path": item.local_path,
            "name": item.name
        }));
    }

    let mut continue_watching = Vec::new();
    for media in Progress::get_continue_watching(&mut tx, user.id, 10).await? {
        let item = match sqlx::query!(
            "SELECT _tblmedia.name, assets.local_path FROM _tblmedia LEFT JOIN assets ON assets.id = _tblmedia.poster
            WHERE _tblmedia.id = ?",
            media
        ).fetch_one(&mut *tx).await {
            Ok(x) => x,
            Err(_) => continue,
        };

        continue_watching.push(json!({
            "id": media,
            "poster_path": item.local_path,
            "name": item.name
        }));
    }

    let continue_watching = if !continue_watching.is_empty() {
        Some(json!({
            "CONTINUE WATCHING": continue_watching,
        }))
    } else {
        None
    };

    Ok(Json(&json!({
        ..?continue_watching,
        "TOP RATED": top_rated,
        "FRESHLY ADDED": recently_added,
    }))
    .into_response())
}
