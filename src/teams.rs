use crate::db::{get_value, set_value, Db};
use crate::helper::elementref_text;
use crate::models::{Player, Team};
use crate::templates::Teams;

use axum::http::StatusCode;
use axum::Extension;
use scraper::{ElementRef, Html, Selector};

const MATCH: &str =
    "https://www.primeleague.gg/leagues/matches/1125918-melo-honigmelonen-vs-slayed-beasts-resolve";

#[axum::debug_handler]
pub(crate) async fn get_teams(Extension(db): Extension<Db>) -> Result<Teams, StatusCode> {
    let value = get_value(Extension(&db), MATCH.to_string()).await;

    let match_document: Html;

    if let Some(value) = value {
        match_document = Html::parse_document(&value);
    } else {
        let match_request_text = reqwest::get(MATCH)
            .await
            .expect("Could not download game")
            .text()
            .await
            .expect("Could not read text from response");

        let value = set_value(
            Extension(&db),
            MATCH.to_string(),
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

    let teams = extract_teams(match_document.clone());
    match teams {
        Ok(teams) => {
            return Ok(Teams { data: teams });
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    // return Ok(Teams { data: teams? });
}

fn extract_teams(match_document: Html) -> Result<Vec<Team>, ()> {
    let mut teams: Vec<Team> = Vec::new();

    let logs_selector = Selector::parse("section.league-match-logs > div > div > div > table.table.table-flex.table-responsive.table-static > tbody > tr").expect("Could not create logs_selector");

    let action_span_selector =
        Selector::parse("td > span").expect("Could not create action_span_selector");

    let team_names = get_team_names(&match_document);

    let split_link = get_split_link(&match_document);
    if split_link.is_none() {
        eprintln!("Could not retrieve split_link from website");
        return Err(());
    }
    let split_link = split_link.unwrap();

    for table_rows in match_document.select(&logs_selector) {
        let players_span_texts = get_players_span_texts(table_rows, &action_span_selector);

        if players_span_texts[2].to_lowercase() == "lineup_submit" {
            let team = get_team(
                &players_span_texts[1],
                &players_span_texts[3],
                &team_names,
                &split_link,
            );
            if let Some(team) = team {
                teams.push(team);
            }
        }
    }

    Ok(teams)
}

fn get_split_link(match_document: &Html) -> Option<&str> {
    let split_selector = Selector::parse(
        "div.page-header-content > div > ul > li.breadcrumbs-item:nth-child(2) > a",
    )
    .expect("Could not create split_selector");

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
    let mut players_span_text: Vec<String> = vec![];

    for span in table_rows.select(&action_span_selector) {
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
) -> Option<Team> {
    let mut players: Vec<Player> = Vec::new();

    let team_index = get_team_index(submitter_text);
    match team_index {
        Some(team_index) => {
            let name = &team_names[team_index];

            let player_texts = span_text.split(',');

            for player_string in player_texts {
                players.push(parse_player(player_string, &split_link));
            }

            return Some(Team {
                name: name.to_string(),
                data: players,
            });
        }
        None => return None,
    }
}

fn get_team_index(submitter_text: &str) -> Option<usize> {
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

fn parse_player(player_string: &str, split_link: &str) -> Player {
    let mut id_name = player_string.trim().split(':');

    let id = id_name.next().unwrap_or("ERR id").to_string();
    let name = id_name.next().unwrap_or("ERR name").to_string();
    let link = format!("{}/users/{}-{}", split_link, id, name);
    let game_account = get_game_account(&link).expect("Could not retrieve game account");

    Player {
        id,
        name,
        link,
        game_account,
    }
}

fn get_team_names(match_document: &Html) -> Vec<String> {
    let team_names_selector = Selector::parse("div.content-match-head-team > div > div > a > h2")
        .expect("Could not create team_names_selector");
    let mut team_names: Vec<String> = Vec::new();

    for team_name in match_document.select(&team_names_selector) {
        team_names.push(elementref_text(&team_name, None));
    }

    team_names
}

fn get_game_account(link: &str) -> Option<String> {
    let user_request_text = tokio::task::block_in_place(move || {
        let user_request = reqwest::blocking::get(link).expect("couldnt get user account page");
        let user_request_text = user_request
            .text()
            .expect("could not get user account page response text");
        return user_request_text;
    });

    let account_page_document = Html::parse_document(&user_request_text);
    let game_account_selector = Selector::parse(
        "ul.quick-info > li > span[title*=\"League of Legends » LoL Summoner Name\"]",
    )
    .expect("could not create game account selector");

    let game_account_element = account_page_document.select(&game_account_selector).next();

    if let Some(game_account_element) = game_account_element {
        return Some(elementref_text(&game_account_element, None));
    }

    None
}
