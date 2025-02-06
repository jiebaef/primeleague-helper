use axum::extract::FromRef;
use sqlx::PgPool;

use crate::database::store::Store;

#[derive(Clone, FromRef)]
pub struct AppState {
    store: Store,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            store: Store::new(pool),
        }
    }
}
