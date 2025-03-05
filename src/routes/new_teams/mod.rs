use crate::appstate::AppState;
use crate::database::store::{CachedResponsesStore, Store};
use crate::helper::{elementref_text, Selectors};
use crate::templates;

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
) -> Result<templates::new_teams::NewTeams, templates::error::Error> {
    let team_url = params.team_url;

    let team_page = CachedResponsesStore::get_or_add(&store.pool, &team_url).await;
    if let Err(e) = team_page {
        eprintln!("{}", e);
        return Err(templates::error::Error {
            err: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
        });
    }

    let team_page = team_page.unwrap();

    let document: Html = Html::parse_document(&team_page.data);

    let team_tag = get_team_tag(&document, &selectors);
    if team_tag.is_none() {
        return Err(templates::error::Error {
            err: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
        });
    }
    let team_tag = team_tag.unwrap();

    let re: regex::Regex = regex::Regex::new(
        r"(?<team1>\w+)\s*(?<win1>\d{0,1})\s*:{0,1}\s*(?<win2>\d{0,1})\s*(?<team2>\w+)",
    )
    .unwrap();
    let matches = get_matches(&document, &selectors, re);
    if matches.is_none() {
        return Err(templates::error::Error {
            err: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
        });
    }
    let matches = matches.unwrap();

    Ok(templates::new_teams::NewTeams {
        tag: team_tag,
        matches,
    })
}

fn get_team_tag(html: &Html, selectors: &Selectors) -> Option<String> {
    let team_tag = html.select(&selectors.team_tag).next();
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

fn get_matches(
    html: &Html,
    selectors: &Selectors,
    re: regex::Regex,
) -> Option<Vec<templates::new_teams::PlMatch>> {
    let matches = html.select(&selectors.team_matches);

    let mut pl_matches = Vec::new();
    for games in matches {
        let val = elementref_text(&games, None).replace("\n", "");
        let x = val.clone();
        println!("{x}");
        if let Some(captures) = re.captures(&x) {
            println!(
                "Team 1: [{},\t{},\t{}]\t\t||\t\tTeam 2 [{},\t{},\t{}]",
                &captures["team1"],
                &captures["win1"],
                &captures["win1"].len(),
                &captures["team2"],
                &captures["win2"],
                &captures["win2"].len()
            );
        }
        println!("");

        pl_matches.push(templates::new_teams::PlMatch { val })
    }

    Some(pl_matches)
}
// pub(crate) async fn get_all_matches(// Query(params): Query<GetTeamsParameters>,
// ) -> Result<NewTeams, String> {
//     eprintln!("NOT IMPLEMENTED");
//     Err("NOT IMPLEMENTED".into())
// }
