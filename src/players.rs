use crate::models::{Player, Team};
use crate::templates::Teams;
use scraper::{ElementRef, Html, Selector};

pub(crate) async fn get_teams() -> Result<Teams, ()> {
    let match_request_text = reqwest::get("https://www.primeleague.gg/leagues/matches/1125918-melo-honigmelonen-vs-slayed-beasts-resolve")
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
    let match_request_text = reqwest::get("https://www.primeleague.gg/leagues/matches/1125918-melo-honigmelonen-vs-slayed-beasts-resolve")
        .await
        .expect("Could not download game")
        .text()
        .await
        .expect("Could not read text from response");

    let match_document = Html::parse_document(&match_request_text);

    let split = extract_split(&match_document);

    Ok(split)
}

fn extract_split(match_document: &Html) -> String {
    let split_selector = Selector::parse("");
    "".into()
}

fn extract_teams(match_document: &Html) -> Vec<Team> {
    let mut teams: Vec<Team> = Vec::new();

    let logs_selector = Selector::parse("section.league-match-logs > div > div > div > table.table.table-flex.table-responsive.table-static > tbody > tr")
        .expect("Could not create logs_selector");

    let action_span_selector =
        Selector::parse("td > span").expect("Could not create action_span_selector");

    for table_rows in match_document.select(&logs_selector) {
        let players_span_texts = get_players_span_texts(table_rows, &action_span_selector);

        if players_span_texts[2].to_lowercase() == "lineup_submit" {
            teams.push(get_team(&players_span_texts[3]));
        }
    }

    teams
}

fn extract_players(match_document: &Html) -> Vec<Vec<Player>> {
    let mut players: Vec<Vec<Player>> = Vec::new();

    let logs_selector = Selector::parse("section.league-match-logs > div > div > div > table.table.table-flex.table-responsive.table-static > tbody > tr")
        .expect("Could not create logs_selector");

    let action_span_selector =
        Selector::parse("td > span").expect("Could not create action_span_selector");

    for table_rows in match_document.select(&logs_selector) {
        let players_span_texts = get_players_span_texts(table_rows, &action_span_selector);

        if players_span_texts[2].to_lowercase() == "lineup_submit" {
            let team = get_team(&players_span_texts[3]);
            players.push(team.data);
        }
    }

    players
}

fn get_players_span_texts(
    table_rows: ElementRef<'_>,
    action_span_selector: &Selector,
) -> Vec<String> {
    let mut players_span_text: Vec<String> = vec![];

    for span in table_rows.select(&action_span_selector) {
        let text = span.text().collect::<Vec<_>>().join(" ");
        players_span_text.push(text);
    }

    players_span_text
}

fn get_team(span_text: &str) -> Team {
    let mut players: Vec<Player> = Vec::new();

    let player_texts = span_text.split(',');

    for player_string in player_texts {
        players.push(parse_player(player_string));
    }

    Team { data: players }
}

fn parse_player(player_string: &str) -> Player {
    let id_name = player_string.trim().split(':').collect::<Vec<_>>();
    Player {
        id: id_name[0].into(),
        name: id_name[1].into(),
    }
}
