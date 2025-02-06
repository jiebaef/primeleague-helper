pub mod index;
pub mod new_teams;
pub mod teams;

use crate::routes::index::get_index;
use axum::{routing::get, Router};
use new_teams::{get_all_matches, get_new_teams};
use teams::get_teams;

pub fn add_routes(router: Router) -> Router {
    let router = router
        .route("/", get(get_index))
        .route("/teams", get(get_teams))
        .route("/new_teams", get(get_new_teams))
        .route("/get_all_matches", get(get_all_matches));
    // .route("/split", get(get_split));

    return router;
}
