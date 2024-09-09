use axum::routing::get;
use axum::Router;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::apis::hello::root;
use crate::app_state::AppState;

pub fn routes(state: AppState) -> Router {
    //
    let tracing_layer = TraceLayer::new_for_http();
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any());

    Router::new()
        .route("/", get(root))
        .layer(tracing_layer)
        .layer(cors_layer)
        .with_state(state)
}
