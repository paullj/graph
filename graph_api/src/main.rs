use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn graph(body: String) -> (StatusCode, String) {
    match graph_core::generate_graph(&body) {
        Ok(result) => (StatusCode::OK, result),
        Err(_) => (StatusCode::NOT_ACCEPTABLE, "Invalid input".to_string()),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/graph", post(graph));

    Ok(router.into())
}
