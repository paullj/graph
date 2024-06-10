use axum::{
    http::{Method, StatusCode},
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

async fn status() -> (StatusCode, String) {
    (StatusCode::OK, String::from("OK"))
}

async fn graph(body: String) -> (StatusCode, String) {
    match graph_core::generate_graph(&body) {
        Ok(result) => (StatusCode::OK, result),
        Err(_) => (StatusCode::NOT_ACCEPTABLE, String::from("Invalid input")),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/api/status", get(status))
        .route("/api/graph", post(graph))
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST])
                    .allow_headers(Any)
                    .allow_credentials(false),
            ),
        );

    Ok(router.into())
}
