use primeleague_helper::db::Db;
use primeleague_helper::helper::init_selectors;
use primeleague_helper::routes::add_routes;

use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Router};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));
    let selectors = init_selectors();

    let router = Router::new();
    let app = add_routes(router)
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
