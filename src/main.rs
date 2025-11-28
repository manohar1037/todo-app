mod auth;
mod config;
mod constant;
mod db;
mod error;
mod handlers;
mod model;
mod router;
mod state;
mod taskrepo;
mod helper;
use std::sync::Arc;
use tokio::net::TcpListener;


use crate::{helper::rewrite_request_uri, router::create_router, state::AppState};
use axum::{ServiceExt};
use tower::Layer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let appstate = Arc::new(AppState::new().await?);

    let router = create_router(appstate.clone());

    let middleware = tower::util::MapRequestLayer::new(rewrite_request_uri);
    let app_with_middleware = middleware.layer(router);
    let port = appstate.port().to_string();
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app_with_middleware.into_make_service()).await?;
    Ok(())
}
