pub mod index;
pub mod new_teams;

use crate::{appstate::AppState, routes::index::get_index};
use axum::{routing::get, Router};
use new_teams::new_teams_router;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_index))
        .merge(new_teams_router())
}
