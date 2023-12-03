use axum::routing::{get, get_service};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct Release {
    timestamp: u64,
    data: String,
}

async fn get_handler() -> Json<Vec<Release>> {
    // Dummy vector of Release structs
    let releases = vec![
        Release {
            timestamp: 1638400000, // Replace with your actual timestamp
            data: "Release 1".to_string(),
        },
        Release {
            timestamp: 1638500000,
            data: "Release 2".to_string(),
        },
    ];

    Json(releases)
}

#[tokio::main]
async fn main() {
    // Setup the Axum app with a single route handling GET requests
    let app = Router::new().route("/", get(get_handler));

    // Run the server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
