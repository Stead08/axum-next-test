use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

pub fn create_router() -> Router{
    let api_router = api_router();
    // let dir_router = Router::new()
    //     .nest_service("/", ServeDir::new("static"));

    Router::new()
        // .nest("/", dir_router)
        .nest("/api", api_router)
}

pub fn api_router() -> Router{
    Router::new()
        .route("/health", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string())
}