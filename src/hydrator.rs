use crate::fpt_client::FPTClient;
use crate::{init_db, unhydrated_iterator::UnhydratedIterator};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
struct AppState {
    fpt_client: FPTClient,
}

pub async fn run(word_map: Arc<Mutex<HashMap<String, String>>>) {
    let shared_state = AppState {
        fpt_client: FPTClient::new(word_map).expect("Config invalid"),
    };

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/receive", post(receive))
        .with_state(shared_state.into());

    println!("Running on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hydrator receiver. Make sure this thing isn't running all night."
}

async fn receive(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<FPTResponse>,
) -> (StatusCode) {
    let fpt_client = _state.clone().fpt_client.clone();
    match fpt_client.try_download(payload.message).await {
        Ok(word) => {
            println!("Successfully downloaded {}", word);
        }
        Err(err) => {
            println!("Error");
        }
    }
    StatusCode::CREATED
}

#[derive(Deserialize)]
struct FPTResponse {
    message: String,
    success: String,
}
