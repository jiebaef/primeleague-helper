use crate::db::{get_value, set_value, Db};
use crate::helper::elementref_text;
use crate::models::{player::Player, team::Team};
use crate::templates::teams::Teams;
use crate::Selectors;

use axum::extract::Query;
use axum::http::StatusCode;
use axum::Extension;
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize)]
pub struct GetTeamsParameters {
    pub match_url: String,
}

#[axum::debug_handler]
pub(crate) async fn get_teams(
    Query(params): Query<GetTeamsParameters>,
    Extension(db): Extension<Db>,
    Extension(selectors): Extension<Selectors>,
) -> Result<Teams, StatusCode> {
    println!(">get_teams()");
    let match_url = params.match_url;

    let value = get_value(Extension(&db), &match_url).await;

    let match_document: Html;

    if let Some(value) = value {
        match_document = Html::parse_document(&value);
    } else {
        let match_request_text = reqwest::get(&match_url)
            .await
            .expect("Could not download game")
            .text()
            .await
            .expect("Could not read text from response");

        let value = set_value(
            Extension(&db),
            match_url.to_string(),
            match_request_text.clone(),
        )
        .await;

        match value {
            Ok(value) => {
                match_document = Html::parse_document(&value);
            }
            Err(e) => {
                eprintln!("{:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    let teams = extract_teams_matchpage(match_document.clone(), selectors.clone());

    println!("\n\n\n");

    match teams {
        Ok(teams) => {
            return Ok(Teams { data: teams });
        }
        Err(e) => {
            if e == "Could not retrieve teams" {
                let teams = extract_teams_teamspage(match_document, selectors);
                match teams {
                    Ok(teams) => {
                        return Ok(Teams { data: teams });
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
            } else {
                eprintln!("{:?}", e);
                return Err(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
}

fn extract_teams_teamspage(
    match_document: Html,
    selectors: Selectors,
) -> Result<Vec<Team>, String> {
    let mut teams: Vec<Team> = Vec::new();

    let team_names = get_team_names(&match_document, selectors.team_names);

    let team_links = get_team_links(&match_document, selectors.team_links);

    let split_link = get_split_link(&match_document, selectors.split_link);
    if split_link.is_none() {
        return Err("Could not retrieve split_link from website".into());
    }
    let split_link = split_link.unwrap();

    todo!("implement fallback");
    for table_rows in match_document.select(&selectors.team_links) {}

    Err("not implemented".into())
}

fn extract_teams_matchpage(
    match_document: Html,
    selectors: Selectors,
) -> Result<Vec<Team>, String> {
    println!(">extract_teams()");
    let mut teams: Vec<Team> = Vec::new();

    let logs_selector = selectors.logs;

    let action_span_selector = selectors.action_span;

    let team_names = get_team_names(&match_document, selectors.team_names);

    let split_link = get_split_link(&match_document, selectors.split_link);
    if split_link.is_none() {
        return Err("Could not retrieve split_link from website".into());
    }
    let split_link = split_link.unwrap();

    for table_rows in match_document.select(&logs_selector) {
        let texts = get_players_span_texts(table_rows, &action_span_selector);

        if texts[2].to_lowercase() == "lineup_submit" {
            let team = get_team(
                &texts[1],
                &texts[3],
                &team_names,
                &split_link,
                &selectors.game_account,
            );
            if let Some(team) = team {
                teams.push(team);
            }
        }
    }

    if teams.len() < 10 {
        return Err("Could not retrieve teams".into());
    }

    Ok(teams)
}

fn get_split_link(match_document: &Html, split_selector: Selector) -> Option<&str> {
    println!(">get_split_link()");
    if let Some(element) = match_document.select(&split_selector).next() {
        if let Some(href) = element.value().attr("href") {
            return Some(href);
        }
    }

    None
}

fn get_players_span_texts(
    table_rows: ElementRef<'_>,
    action_span_selector: &Selector,
) -> Vec<String> {
    println!(">get_players_span_texts()");
    let mut players_span_text: Vec<String> = vec![];

    for span in table_rows.select(action_span_selector) {
        let text = elementref_text(&span, None);
        players_span_text.push(text);
    }

    players_span_text
}

fn get_team(
    submitter_text: &str,
    span_text: &str,
    team_names: &Vec<String>,
    split_link: &str,
    game_account_selector: &Selector,
) -> Option<Team> {
    println!(">get_team()");
    let mut players: Vec<Player> = Vec::new();

    let team_index = get_team_index(submitter_text);
    match team_index {
        Some(team_index) => {
            let name = &team_names[team_index];

            let player_texts = span_text.split(',');

            for player_string in player_texts {
                players.push(parse_player(
                    player_string,
                    &split_link,
                    &game_account_selector,
                ));
            }

            let accounts = players
                .iter()
                .map(|player| player.game_account.as_str())
                .collect::<Vec<&str>>()
                .join(",");
            let opgg = format!(
                "https://www.op.gg/multisearch/euw?summoners={}",
                encode(&accounts)
            );

            return Some(Team {
                name: name.to_string(),
                data: players,
                opgg,
            });
        }
        None => return None,
    }
}

fn get_team_index(submitter_text: &str) -> Option<usize> {
    println!(">get_team_index()");
    const SEARCH_TERM: &str = "Team ";
    if let Some(start_index) = submitter_text.find(&SEARCH_TERM) {
        let number_index = start_index + SEARCH_TERM.len();
        if let Some(end_index) = submitter_text[number_index..].find(")") {
            let number_str = &submitter_text[number_index..number_index + end_index];
            return number_str.parse::<usize>().ok().map(|n| n - 1);
        }
    }
    None
}

fn parse_player(player_string: &str, split_link: &str, game_account_selector: &Selector) -> Player {
    println!(">parse_player()");
    let mut id_name = player_string.trim().split(':');

    let id = id_name.next().unwrap_or("ERR id").to_string();
    let name = id_name.next().unwrap_or("ERR name").to_string();
    let link = format!("{}/users/{}-{}", split_link, id, name);
    let game_account =
        get_game_account(&link, &game_account_selector).expect("Could not retrieve game account");

    Player {
        id,
        name,
        link,
        game_account,
    }
}

fn get_team_names(match_document: &Html, team_names_selector: Selector) -> Vec<String> {
    println!(">get_team_names()");
    let mut team_names: Vec<String> = Vec::new();

    for team_name in match_document.select(&team_names_selector) {
        team_names.push(elementref_text(&team_name, None));
    }

    team_names
}

fn get_team_links(match_document: &Html, team_links_selector: Selector) -> Vec<String> {
    println!(">get_team_links()");
    let mut team_links: Vec<String> = Vec::new();

    for team_link in match_document.select(&team_links_selector) {
        team_links.push(elementref_text(&team_link, None));
    }

    team_links
}

fn get_game_account(link: &str, game_account_selector: &Selector) -> Option<String> {
    println!(">get_game_account()");
    let user_request_text = tokio::task::block_in_place(move || {
        let user_request = reqwest::blocking::get(link).expect("couldnt get user account page");
        let user_request_text = user_request
            .text()
            .expect("could not get user account page response text");
        return user_request_text;
    });

    let account_page_document = Html::parse_document(&user_request_text);

    let game_account_element = account_page_document.select(game_account_selector).next();

    if let Some(game_account_element) = game_account_element {
        return Some(elementref_text(&game_account_element, None));
    }

    None
}
