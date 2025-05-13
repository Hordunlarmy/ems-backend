use axum::{
    routing::get,
    Router,
};
use dotenvy::dotenv;
use std::env;
use tracing::info;
use tracing_subscriber;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    let app = Router::new().route("/", get(root_handler));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await.expect("Failed to bind address");

    info!("🚀 Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, World!"
}

