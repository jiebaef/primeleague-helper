use axum::Extension;
use primeleague_helper::appstate::AppState;
use primeleague_helper::db::Db;
use primeleague_helper::helper::init_selectors;
use primeleague_helper::routes::get_router;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&db_url).await?;

    let db: Db = Arc::new(RwLock::new(HashMap::new()));
    let selectors = init_selectors();

    let app = get_router()
        .nest_service("/css", ServeDir::new("static/css"))
        .layer(Extension(db))
        .layer(Extension(selectors))
        .with_state(AppState::new(pool));

    let tcp_listener = TcpListener::bind(&"0.0.0.0:42069")
        .await
        .expect("couldn't bind to port 42069");

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .expect("couldn't host server");

    Ok(())
}
