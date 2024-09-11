use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::apis::image::upload::upload_image;
use crate::apis::user::{login, register};
use crate::app_state::AppState;
use crate::middleware::auth::auth_middleware;

pub fn routes(state: AppState) -> Router {
    //
    let tracing_layer = TraceLayer::new_for_http();
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
