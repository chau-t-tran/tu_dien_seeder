use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
pub async fn run() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/receive", post(receive));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hydrator receiver. Make sure this thing isn't running all night."
}

async fn receive(Json(payload): Json<FPTResponse>) -> (StatusCode) {
    (StatusCode::OK)
}

#[derive(Deserialize)]
struct FPTResponse {
    message: String,
    success: String,
}
