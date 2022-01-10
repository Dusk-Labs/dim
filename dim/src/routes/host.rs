use crate::core::DbConnection;
use crate::errors;
use database::user::User;
use warp::reply;
use crate::json;

pub async fn admin_exists(conn: DbConnection) -> Result<impl warp::Reply, errors::DimError> {
    let mut tx = conn.read().begin().await?;
    Ok(reply::json(&json!({
        "exists": !User::get_all(&mut tx).await?.is_empty()
    })))
}

#[doc(hidden)]
pub(crate) mod filters {
    use crate::core::DbConnection;
    use warp::reject;
    use warp::Filter;

    use super::super::global_filters::with_state;

    pub fn admin_exists(
        conn: DbConnection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api" / "v1" / "host" / "admin_exists")
            .and(warp::get())
            .and(with_state(conn))
            .and_then(|conn: DbConnection| async move {
                super::admin_exists(conn)
                    .await
                    .map_err(|e| reject::custom(e))
            })
    }
}
