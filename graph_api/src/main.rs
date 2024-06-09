use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

async fn status() -> &'static str {
    "Ok!"
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
        .nest_service("/", ServeDir::new("build"))
        .route("/api", get(status))
        .route("/api/graph", post(graph));

    Ok(router.into())
}
