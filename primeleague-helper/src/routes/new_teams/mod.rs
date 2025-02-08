use crate::appstate::AppState;
use crate::database::store::{CachedResponsesStore, Store};
use crate::helper::{elementref_text, Selectors};
use crate::templates;
use crate::templates::new_teams::NewTeams;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Extension;
use axum::{extract::Query, routing::get, Router};
use scraper::Html;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTeamsParameters {
    pub team_url: String,
}

pub fn new_teams_router() -> Router<AppState> {
    Router::new().route("/new_teams", get(get_new_teams))
    // .route("/new_teams/get_all_matches", get(get_all_matches))
}

pub(crate) async fn get_new_teams(
    Query(params): Query<GetTeamsParameters>,
    Extension(selectors): Extension<Selectors>,
    State(store): State<Store>,
) -> Result<String, templates::error::Error> {
    let team_url = params.team_url;

    let response = CachedResponsesStore::get_or_add(&store.pool, &team_url).await;
    if let Err(e) = response {
        eprintln!("{}", e);
        return Err(templates::error::Error {
            err: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
        });
    }

    let res = response.unwrap();

    let team_tag = get_team_tag(res.data, selectors);
    if team_tag.is_none() {
        return Err(templates::error::Error {
            err: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
        });
    }
    let team_tag = team_tag.unwrap();

    Ok(format!("{}", team_tag))
}

fn get_team_tag(html_string: String, selectors: Selectors) -> Option<String> {
    let document: Html = Html::parse_document(&html_string);

    let team_tag = document.select(&selectors.team_tag).next();
    if team_tag.is_none() {
        return None;
    }

    let text = elementref_text(&team_tag.unwrap(), None);
    let team_tag = text.split(": ").nth(1);
    if team_tag.is_none() {
        eprintln!("could not parse team_tag");
        return None;
    }
    Some(team_tag.unwrap().into())
}

// pub(crate) async fn get_all_matches(// Query(params): Query<GetTeamsParameters>,
// ) -> Result<NewTeams, String> {
//     eprintln!("NOT IMPLEMENTED");
//     Err("NOT IMPLEMENTED".into())
// }
