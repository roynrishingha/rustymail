#![allow(unused)]

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, IntoMakeService},
    Json, Router, Server,
};
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;
use tracing::debug;

static HOST: [u8; 4] = [127, 0, 0, 1];

async fn health_check() -> Response {
    StatusCode::OK.into_response()
}

pub fn run_app(port: u16) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    let app = Router::new().route("/health_check", get(health_check));

    let addr = SocketAddr::from((HOST, port));
    debug!("listening on {}", addr);

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    Ok(server)
}
