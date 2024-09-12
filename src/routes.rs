use axum::extract::DefaultBodyLimit;
use axum::http::Request;
use axum::routing::post;
use axum::Router;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::{TraceLayer, DefaultMakeSpan};
use tower_http::classify::ServerErrorsFailureClass;

use crate::apis::image::idphoto::upload_image;
use crate::apis::user::{login, register};
use crate::app_state::AppState;
use crate::middleware::auth::auth_middleware;

pub fn routes(state: AppState) -> Router {
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(|request: &Request<_>, _span: &tracing::Span| {
            tracing::info!("请求: {} {}", request.method(), request.uri());
        })
        .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
            tracing::info!("响应: {} 耗时: {:?}", response.status(), latency);
        })
        .on_failure(|error: ServerErrorsFailureClass, latency: std::time::Duration, span: &tracing::Span| {
            let path = span.metadata().map(|m| m.target()).unwrap_or("未知路径");
            
            let error_message = error.to_string();

            tracing::error!(
                path = %path,
                error = %error_message,
                latency = ?latency,
                "API 请求失败"
            );
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
