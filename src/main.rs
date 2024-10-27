mod index;
mod players;
mod routes;
mod templates;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const TEAM: &str = "HOME";

#[tokio::main]
async fn main() {
    let router = Router::new();
    let app = routes::add_routes(router).nest_service("/css", ServeDir::new("static/css"));

    let tcp_listener = TcpListener::bind(&"0.0.0.0:42069")
        .await
        .expect("couldn't bind to port 42069");

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .expect("couldn't host server");
}
