use crate::index::get_index;
use crate::players::{get_split, get_teams};
use axum::{routing::get, Router};

pub(crate) fn add_routes(router: Router) -> Router {
    let router = router
        .route("/", get(get_index))
        .route("/teams", get(get_teams))
        .route("/split", get(get_split));

    return router;
}
