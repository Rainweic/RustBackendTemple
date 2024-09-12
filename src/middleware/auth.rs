use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, State},
    http::{request::Parts, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i32,
}

fn extract_token(auth_header: Option<&str>) -> Result<&str, (StatusCode, String)> {
    auth_header
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "缺少认证token".to_string()))
}

fn validate_token(token: &str, jwt_secret: &[u8]) -> Result<Claims, (StatusCode, String)> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
    .map_err(|_| (StatusCode::UNAUTHORIZED, "无效的token".to_string()))
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let app_state = parts.extensions.get::<AppState>().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "无法访问应用状态".to_string(),
        ))?;

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok());

        let token = extract_token(auth_header)?;
        let claims = validate_token(token, app_state.jwt_secret.as_ref())?;

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let token = extract_token(auth_header)?;
    let claims = validate_token(token, state.jwt_secret.as_ref())?;

    let auth_user = AuthUser {
        user_id: claims.sub,
    };
    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}
