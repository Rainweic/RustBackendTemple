use crate::db::DbConnPool;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub dbcp: Arc<DbConnPool>,
}

impl AppState {
    pub fn new(dbcp: DbConnPool) -> Self {
        Self {
            dbcp: Arc::new(dbcp),
        }
    }
}
