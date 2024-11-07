use axum::Extension;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) type Db = Arc<RwLock<HashMap<String, String>>>;

pub(crate) async fn get_value(Extension(ref db): Extension<&Db>, key: &str) -> Option<String> {
    // println!("DB,GET,{}", key);
    let db = db.read().await;

    match db.get(key) {
        Some(value) => return Some(value.to_string()),
        None => None,
    }
}

pub(crate) async fn set_value(
    Extension(ref db): Extension<&Db>,
    key: String,
    value: String,
) -> Result<String, ()> {
    // println!("DB,SET,{}", key);
    let mut db = db.write().await;
    db.insert(key, value.clone());

    Ok(value)
}
