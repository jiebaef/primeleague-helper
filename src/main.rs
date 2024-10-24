mod templates;

use axum::{routing::get, Router};
use templates::Index;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/css", ServeDir::new("static/css"));
    // .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"));

    let tcp_listener = TcpListener::bind(&"0.0.0.0:42069")
        .await
        .expect("couldn't bind to port 42069");

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .expect("couldn't host server");
}

async fn index() -> Index {
    Index {}
}
