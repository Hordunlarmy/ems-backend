use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use dotenvy::dotenv;
use std::env;
use tracing::info;
use tracing_subscriber;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

mod shared;
use shared::response::APIResponse;
use shared::database::Database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check_handler))
        .layer(middleware_stack);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    info!("🚀 Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> impl IntoResponse {
    let response = APIResponse::success("Hello, world!", None);
    (StatusCode::OK, Json(response))
}

async fn health_check_handler() -> impl IntoResponse {
    let response = APIResponse::success("OK", None);
    (StatusCode::OK, Json(response))
}

