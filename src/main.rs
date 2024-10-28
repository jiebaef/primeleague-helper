mod db;
mod helper;
mod index;
mod models;
mod routes;
mod teams;
mod templates;

use crate::db::Db;

use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Router};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

const TEAM: &str = "HOME";

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));

    let router = Router::new();
    let app = routes::add_routes(router)
        .nest_service("/css", ServeDir::new("static/css"))
        .layer(Extension(db));

    let tcp_listener = TcpListener::bind(&"0.0.0.0:42069")
        .await
        .expect("couldn't bind to port 42069");

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .expect("couldn't host server");
}
