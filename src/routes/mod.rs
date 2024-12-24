pub mod index;
pub mod teams;

use crate::routes::index::get_index;
use axum::{routing::get, Router};
use teams::get_teams;

pub fn add_routes(router: Router) -> Router {
    let router = router
        .route("/", get(get_index))
        .route("/teams", get(get_teams));
    // .route("/split", get(get_split));

    return router;
}
