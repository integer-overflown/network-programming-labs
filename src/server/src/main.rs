use axum::{routing::get, Router};

use tracing::info;

const ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    info!("Listening on {ADDRESS}");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World"
}
