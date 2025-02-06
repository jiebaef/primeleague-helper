// use crate::db::{get_value, set_value, Db};
// use crate::helper::{elementref_text, Selectors};
// use crate::models::{player::Player, team::Team};
use crate::templates::new_teams::NewTeams;
// use crate::templates::teams::Teams;

use axum::extract::Query;
use axum::http::StatusCode;
// use axum::Extension;
// use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
// use urlencoding::encode;

#[derive(Deserialize)]
pub struct GetTeamsParameters {
    pub team_url: String,
    pub my_team_name: String,
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
) -> Result<String, StatusCode> {
    let team_url = params.team_url;
    println!("{}\n", team_url);

    let team_text = reqwest::get(team_url)
        .await
        .expect("Could not download game")
        .text()
        .await
        .expect("Could not read text from response");
    println!("{}", team_text);

    Ok("idk if this works".into())
}
