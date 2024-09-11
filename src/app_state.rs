use crate::db::DbConnPool;
use rand::Rng;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DbConnPool>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(db: DbConnPool) -> Self {
        Self {
            db: Arc::new(db),
            // 生成随机字符串
            jwt_secret: rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(32)
                .map(char::from)
                .collect(),
        }
    }
}
