use crate::models::{Player, Team};
use crate::templates::Teams;
use scraper::{ElementRef, Html, Selector};

const MATCH: &str =
    "https://www.primeleague.gg/leagues/matches/1125918-melo-honigmelonen-vs-slayed-beasts-resolve";

pub(crate) async fn get_teams() -> Result<Teams, ()> {
    let match_request_text = reqwest::get(MATCH)
        .await
        .expect("Could not download game")
        .text()
        .await
        .expect("Could not read text from response");

    let match_document = Html::parse_document(&match_request_text);

    let teams = extract_teams(&match_document);

    return Ok(Teams { data: teams });
}

pub(crate) async fn get_split() -> Result<String, ()> {
    let match_request_text = reqwest::get(MATCH)
        .await
        .expect("Could not download game")
        .text()
        .await
        .expect("Could not read text from response");

    let match_document = Html::parse_document(&match_request_text);

    let split = extract_split(&match_document);

    if let Some(split) = split {
        return Ok(split.to_string());
    }

    Err(())
}

fn extract_split(match_document: &Html) -> Option<&str> {
    let split_selector = Selector::parse(
        "div.page-header-content > div > ul > li.breadcrumbs-item:nth-child(2) > a",
    )
    .expect("Could not create split_selector");

    for element in match_document.select(&split_selector) {
        if let Some(href) = element.value().attr("href") {
            return Some(href);
        }
    }

    return None;
}

fn extract_teams(match_document: &Html) -> Vec<Team> {
    let mut teams: Vec<Team> = Vec::new();

    let logs_selector = Selector::parse("section.league-match-logs > div > div > div > table.table.table-flex.table-responsive.table-static > tbody > tr").expect("Could not create logs_selector");

    let action_span_selector =
        Selector::parse("td > span").expect("Could not create action_span_selector");

    let team_names = get_team_names(&match_document);

    for table_rows in match_document.select(&logs_selector) {
        let players_span_texts = get_players_span_texts(table_rows, &action_span_selector);

        if players_span_texts[2].to_lowercase() == "lineup_submit" {
            if let Some(team) =
                get_team(&players_span_texts[1], &players_span_texts[3], &team_names)
            {
                teams.push(team);
            }
        }
    }

    teams
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

fn get_team(submitter_text: &str, span_text: &str, team_names: &Vec<String>) -> Option<Team> {
    let mut players: Vec<Player> = Vec::new();

    let team_index = get_team_index(submitter_text);
    match team_index {
        Some(team_index) => {
            let name = &team_names[team_index];

            let player_texts = span_text.split(',');

            for player_string in player_texts {
                players.push(parse_player(player_string));
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

fn parse_player(player_string: &str) -> Player {
    let id_name = player_string.trim().split(':').collect::<Vec<_>>();
    Player {
        id: id_name[0].into(),
        name: id_name[1].into(),
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

fn elementref_text(element: &ElementRef<'_>, join_by: Option<&str>) -> String {
    return element
        .text()
        .collect::<Vec<_>>()
        .join(join_by.unwrap_or(" "));
}
