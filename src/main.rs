mod db;
mod helper;
mod models;
mod routes;
mod templates;

use crate::db::Db;
use crate::helper::init_selectors;

use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Router};
use scraper::Selector;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub(crate) struct Selectors {
    pub(crate) logs: Selector,
    pub(crate) action_span: Selector,
    pub(crate) split_link: Selector,
    pub(crate) team_names: Selector,
    pub(crate) team_links: Selector,
    pub(crate) team_participants: Selector,
    pub(crate) game_account: Selector,
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));
    let selectors = init_selectors();

    let router = Router::new();
    let app = routes::add_routes(router)
        .nest_service("/css", ServeDir::new("static/css"))
        .layer(Extension(db))
        .layer(Extension(selectors));

    let tcp_listener = TcpListener::bind(&"0.0.0.0:42069")
        .await
        .expect("couldn't bind to port 42069");

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .expect("couldn't host server");
}
