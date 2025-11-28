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
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::{router::create_router, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let appstate = Arc::new(AppState::new().await?);

    let router = create_router(appstate.clone());
    let port = appstate.port().to_string();

    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, router).await?;
    Ok(())
}
