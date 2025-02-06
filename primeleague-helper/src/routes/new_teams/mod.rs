use crate::appstate::AppState;
use crate::database::store::{CachedResponsesStore, Store};
use crate::templates::new_teams::NewTeams;

use axum::extract::State;
use axum::http::StatusCode;
use axum::{extract::Query, routing::get, Router};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTeamsParameters {
    pub team_url: String,
    pub my_team_name: String,
}

pub fn new_teams_router() -> Router<AppState> {
    Router::new()
        .route("/new_teams", get(get_new_teams))
        .route("/new_teams/get_all_matches", get(get_all_matches))
}

pub(crate) async fn get_new_teams(
    Query(params): Query<GetTeamsParameters>,
    // Extension(db): Extension<Db>,
    // Extension(selectors): Extension<Selectors>,
) -> Result<NewTeams, StatusCode> {
    println!("{:?}, {:?}", params.my_team_name, params.team_url);
    println!("https://www.primeleague.gg/de/leagues/prm/3181-spring-split-202425/teams/206205-melo-honigmelonen");
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub(crate) async fn get_all_matches(
    Query(params): Query<GetTeamsParameters>,
    State(store): State<Store>,
) -> Result<String, StatusCode> {
    let team_url = params.team_url;
    // println!("{}\n", team_url);

    let res = CachedResponsesStore::get(store.pool, team_url.clone()).await;
    match res {
        Ok(res) => println!("OK: {:?}: {:?}", team_url, res),
        Err(e) => eprintln!("ERR: {:?}: {:?}", team_url, e),
    }

    // let team_text = reqwest::get(team_url)
    //     .await
    //     .expect("Could not download game")
    //     .text()
    //     .await
    //     .expect("Could not read text from response");
    // println!("{}", team_text);

    Ok("idk if this works".into())
}
