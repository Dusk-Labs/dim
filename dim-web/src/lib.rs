use std::net::SocketAddr;

pub mod routes;
pub mod tree;

pub use axum;
use axum::ServiceExt;

#[inline]
pub async fn serve(addr: &SocketAddr, router: axum::Router) -> Result<(), hyper::Error> {
    axum::Server::bind(addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
}
