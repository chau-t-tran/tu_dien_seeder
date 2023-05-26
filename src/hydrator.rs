use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::time::Duration;
use tokio::time::sleep;

use crate::{init_db, unhydrated_iterator::UnhydratedIterator};

pub async fn run() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/receive", post(receive));

    println!("Running on 0.0.0.0:3000");
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

pub async fn iterate_unhydrated() {
    let mut conn = init_db().unwrap();
    let untranslated = UnhydratedIterator::new(&mut conn);
    let duration = Duration::new(1, 500);

    println!("Iterating through words...");
    for text in untranslated {
        println!("{}", text);
        sleep(duration).await;
    }
}
