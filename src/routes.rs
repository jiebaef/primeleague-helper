use crate::index::get_index;
use crate::players::get_players;
use axum::{routing::get, Router};

pub(crate) fn add_routes(router: Router) -> Router {
    let router = router
        .route("/", get(get_index))
        .route("/players", get(get_players));

    return router;
}
