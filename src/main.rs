mod db;
mod helper;
mod models;
mod routes;
mod templates;

use crate::db::Db;

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

    let selectors = Selectors {
        logs: Selector::parse("section.league-match-logs > div > div > div > table.table.table-flex.table-responsive.table-static > tbody > tr").expect("Could not create logs_selector"),
        action_span: Selector::parse("td > span").expect("Could not create action_span_selector"),
        split_link: Selector::parse("div.page-header-content > div > ul > li.breadcrumbs-item:nth-child(2) > a",).expect("Could not create split_selector"),
        team_names: Selector::parse("div.content-match-head-team > div > div > a > h2").expect("Could not create team_names_selector"),
        team_links: Selector::parse("div.content-match-head-team-titles > a").expect("Could not create team_names_selector"),
        team_participants: Selector::parse("").expect("Could not create team_names_selector"),
        game_account: Selector::parse("ul.quick-info > li > span[title*=\"League of Legends » LoL Summoner Name\"]",).expect("could not create game account selector"),
    };

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
