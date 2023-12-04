use api::releases::ReleasesController;
use axum::{routing::get, Json, Router};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

mod api;
mod utils;

#[tokio::main]
async fn main() {
    // Setup the Axum app with a single route handling GET requests
    let app = Router::new()
        // .route("/news", get(get_news))
        .route("/releases", get(get_releases));

    // Run the server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct Release {
    time_str: String,
    title: String,

    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
    date_str: String,
}

async fn get_releases() -> Json<Vec<Release>> {
    // Dummy vector of Release structs
    let releases = ReleasesController::list_releases_today().await;

    match releases {
        Ok(releases) => releases,
        Err(_) => panic!("Error getting releases"),
    }
}
