use anyhow::Result;
use axum::{
    body,
    http::header::CONTENT_TYPE,
    response::Response,
    routing::get,
    routing::{get_service, post},
    Router,
};
use log::info;
use prometheus::{Encoder, TextEncoder};
use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};
use tower_http::{
    compression::{CompressionLayer, DefaultPredicate},
    services::ServeFile,
};

use crate::handlers::index;

mod handlers;

async fn healthcheck() -> &'static str {
    "OK"
}

pub async fn run_server() -> Result<()> {
    env_logger::try_init().ok();
    info!("starting the application");

    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true)
        .compress_when(DefaultPredicate::new());

    let middleware = tower::ServiceBuilder::new();

    let app = Router::new()
        .route("/", post(index))
        .route("/health", get(healthcheck))
        .route("/metrics", get(metrics))
        .route(
            "/robots.txt",
            get_service(ServeFile::new("./static/robots.txt")),
        )
        .layer(comression_layer)
        .layer(middleware);

    let addr: SocketAddr = (
        IpAddr::from_str(&env::var("HOST").unwrap_or("::".into()))?,
        env::var("PORT").unwrap_or("3030".into()).parse::<u16>()?,
    )
        .into();

    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn metrics() -> Response {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "text/plain; charset=us-ascii")
        .body(body::boxed(body::Full::from(buffer)))
        .unwrap()
}
