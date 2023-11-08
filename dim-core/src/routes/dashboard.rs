use crate::core::DbConnection;
use crate::errors;
use crate::json;

use dim_database::episode::Episode;
use dim_database::genre::*;
use dim_database::library::MediaType;
use dim_database::media::Media;
use dim_database::mediafile::MediaFile;
use dim_database::progress::Progress;

use dim_database::user::User;
use serde_json::Value;

use warp::reply;

pub mod filters {
    use dim_database::DbConnection;

    use dim_database::user::User;
    use warp::reject;
    use warp::Filter;

    use crate::routes::global_filters::with_auth;

    use super::super::global_filters::with_state;

    pub fn dashboard(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "dashboard")
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|user: User, conn: DbConnection| async move {
                super::dashboard(conn, user, tokio::runtime::Handle::current())
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }

    pub fn banners(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "dashboard" / "banner")
            .and(warp::get())
            .and(with_auth(conn.clone()))
            .and(with_state::<DbConnection>(conn))
            .and_then(|user: User, conn: DbConnection| async move {
                super::banners(conn, user)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}

pub async fn dashboard(
    conn: DbConnection,
    user: User,
    _rt: tokio::runtime::Handle,
) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;

    let mut top_rated = Vec::new();
    for media in Media::get_top_rated(&mut tx, 10).await? {
        let item = match sqlx::query!(
            "SELECT _tblmedia.name, assets.local_path FROM _tblmedia LEFT JOIN assets ON assets.id = _tblmedia.poster
            WHERE _tblmedia.id = ?",
            media
        ).fetch_one(&mut tx).await {
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
        ).fetch_one(&mut tx).await {
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
        ).fetch_one(&mut tx).await {
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

    Ok(reply::json(&json!({
        ..?continue_watching,
        "TOP RATED": top_rated,
        "FRESHLY ADDED": recently_added,
    })))
}

pub async fn banners(conn: DbConnection, user: User) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
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

    Ok(reply::json(&banners.iter().take(3).collect::<Vec<_>>()))
}

async fn banner_for_movie(
    conn: &mut dim_database::Transaction<'_>,
    user: &User,
    media: &Media,
) -> Result<Value, errors::DimError> {
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
) -> Result<Value, errors::DimError> {
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
