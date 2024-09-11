use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i32, // 用户ID
    exp: i64, // 过期时间
}

pub async fn register(
    State(state): State<AppState>,
    Json(credentials): Json<UserCredentials>,
) -> Result<StatusCode, (StatusCode, String)> {
    // 检查用户名是否已存在
    let existing_user = sqlx::query("SELECT id FROM users WHERE username = $1")
        .bind(credentials.username.clone())
        .fetch_optional(state.db.as_ref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing_user.is_some() {
        return Err((StatusCode::BAD_REQUEST, "用户名已存在".to_string()));
    }

    // 对密码进行哈希处理
    let hashed_password = hash(credentials.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 将新用户插入数据库
    sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
        .bind(credentials.username)
        .bind(hashed_password)
        .execute(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}

pub async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<UserCredentials>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    // 从数据库中获取用户信息
    let row = sqlx::query("SELECT id, password FROM users WHERE username = $1")
        .bind(credentials.username)
        .fetch_optional(state.db.as_ref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let row = row.ok_or((StatusCode::UNAUTHORIZED, "用户名或密码错误".to_string()))?;

    // 验证密码
    let user_id: i32 = row
        .try_get("id")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let user_password: String = row
        .try_get("password")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let is_valid = verify(credentials.password, &user_password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "用户名或密码错误".to_string()));
    }

    // 生成JWT token
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(LoginResponse { token }))
}
