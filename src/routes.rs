use axum::extract::DefaultBodyLimit;
use axum::http::Request;
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::apis::image::idphoto::upload_image;
use crate::apis::user::{login, register};
use crate::app_state::AppState;
use crate::middleware::auth::auth_middleware;

pub fn routes(state: AppState) -> Router {
    //
    let tracing_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, _span: &tracing::Span| {
            tracing::debug!("Request: {} {}", request.method(), request.uri());
        })
        .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
            let status = response.status();
            let level = if status.is_server_error() {
                Level::ERROR
            } else if status.is_client_error() {
                Level::WARN
            } else {
                Level::INFO
            };
            tracing::info!(level = ?level, status = ?status, latency = ?latency, "响应");
        });

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any());

    Router::new()
        .route("/user/register", post(register))
        .route("/user/login", post(login))
        .nest(
            "/",
            Router::new()
                .route("/image/upload", post(upload_image))
                .layer(DefaultBodyLimit::max(12 * 1024))
                .layer(axum::middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .layer(tracing_layer)
        .layer(cors_layer)
        .with_state(state)
}
