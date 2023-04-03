use std::net::SocketAddr;

pub mod routes;
pub mod tree;

pub use axum;

#[inline]
pub async fn serve(addr: &SocketAddr, router: axum::Router) -> Result<(), hyper::Error> {
    axum::Server::bind(addr)
        .serve(router.into_make_service())
        .await
}
