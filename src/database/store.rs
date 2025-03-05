use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

#[derive(Clone)]
pub struct Store {
    pub pool: PgPool,
}
#[derive(Clone)]
pub struct CachedResponsesStore {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub id: i32,
    pub url: String,
    pub data: String,
}

impl Store {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_response_get_all(
        &self,
        url: String,
        data: String,
    ) -> Result<Vec<CachedResponse>, sqlx::Error> {
        query_as!(
            CachedResponse,
            r#"
            with inserted as (
                insert into primeleague.cached_responses (url, data)
                values ($1, $2)
                returning id, url, data
            )
            select id "id!", url "url!", data "data!"
            from primeleague.cached_responses
            union all
            select id "id!", url "url!", data "data!" from inserted
            order by "id!"
            "#,
            url,
            data
        )
        .fetch_all(&self.pool)
        .await
    }
}

impl CachedResponsesStore {
    pub async fn get(pool: &PgPool, url: &str) -> Result<CachedResponse, sqlx::Error> {
        query_as!(
            CachedResponse,
            r#"
            SELECT id "id!", url "url!", data "data!"
            FROM primeleague.cached_responses
            WHERE url = $1;
            "#,
            url
        )
        .fetch_one(pool)
        .await
    }

    pub async fn add(
        pool: &PgPool,
        url: &str,
        data: String,
    ) -> Result<CachedResponse, sqlx::Error> {
        query_as!(
            CachedResponse,
            r#"
            with inserted as (
                insert into primeleague.cached_responses (url, data)
                values ($1, $2)
                returning id, url, data
            )
            select id "id!", url "url!", data "data!"
            from primeleague.cached_responses
            union all
            select id "id!", url "url!", data "data!" from inserted
            order by "id!"
            "#,
            url,
            data
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_or_add(pool: &PgPool, url: &str) -> Result<CachedResponse, sqlx::Error> {
        let response = Self::get(pool, url).await;

        match response {
            Ok(res) => Ok(res),
            Err(_) => {
                let team_text = reqwest::get(url)
                    .await
                    .expect("Could not download game")
                    .text()
                    .await
                    .expect("Could not read text from response");
                return Self::add(pool, url, team_text).await;
            }
        }
    }
}
