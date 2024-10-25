mod templates;

use axum::{routing::get, Router};
use scraper::{Html, Selector};
use templates::{Index, Player, Players, Teams};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const TEAM: &str = "HOME";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/players", get(players))
        // .route("/teams", get(teams))
        .nest_service("/css", ServeDir::new("static/css"));
    // .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"));

    let tcp_listener = TcpListener::bind(&"0.0.0.0:42069")
        .await
        .expect("couldn't bind to port 42069");

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .expect("couldn't host server");
}

async fn index() -> Index {
    Index {}
}

#[axum::debug_handler]
async fn players() -> Result<Players, ()> {
    // let match_request = reqwest::get("https://www.primeleague.gg/leagues/matches/1125918-melo-honigmelonen-vs-slayed-beasts-resolve").await;
    let match_request_text = reqwest::get("https://www.primeleague.gg/leagues/matches/1125918-melo-honigmelonen-vs-slayed-beasts-resolve").await.expect("Could not download game").text().await.expect("Could not read text from response");
    // let match_request = reqwest::blocking::get("https://www.primeleague.gg/leagues/matches/1125918-melo-honigmelonen-vs-slayed-beasts-resolve");

    let mut players: Vec<Player> = vec![];

    // match match_request {
    //     Ok(match_request) => {
    // let match_request_text = match_request.text().await;
    // let match_request_text = match_request.text();
    // match match_request_text {
    //     Ok(match_request_text) => {
    let match_document = Html::parse_document(&match_request_text);

    let logs_selector = Selector::parse(
                        "section.league-match-logs > div > div > div > table.table.table-flex.table-responsive.table-static > tbody > tr",
                    ).expect("Could not create logs_selector");

    let action_span_selector =
        Selector::parse("td > span").expect("Could not create action_span_selector");

    // match logs_selector {
    //     Ok(logs_selector) => match action_span_selector {
    //         Ok(action_span_selector) => {
    for table_rows in match_document.select(&logs_selector) {
        let mut players_span_text: Vec<String> = vec![];

        for span in table_rows.select(&action_span_selector) {
            let text = span.text().collect::<Vec<_>>().join(" ");
            players_span_text.push(text);
        }

        if players_span_text[2].to_lowercase() == "lineup_submit" {
            let player_texts = players_span_text[3].split(',');
            for player in player_texts {
                let id_name = player.trim().split(':').collect::<Vec<_>>();

                players.push(Player {
                    id: id_name[0].into(),
                    name: id_name[1].into(),
                });
            }
        }
        // }
        //     }
        //     Err(e) => {
        //         eprintln!("{:?}", e);
        //         return Err(());
        //     }
        // },
        // Err(e) => {
        //     eprintln!("{:?}", e);
        //     return Err(());
        // }
    }
    // }
    // Err(e) => {
    //     eprintln!("{:?}", e);
    //     return Err(());
    // }
    //     }
    // }
    // Err(e) => {
    //     eprintln!("{:?}", e);
    //     return Err(());
    // }
    // }

    return Ok(Players { data: players });
}

// async fn teams() -> Result<Teams, ()> {
//     let match_request = reqwest::get("https://www.primeleague.gg/leagues/matches/1125918-melo-honigmelonen-vs-slayed-beasts-resolve").await;
//
//     let teams_selector = Selector::parse("div.content-match-head.content-league-match-head > div.content-match-head-team > div > div.content-match-head-team-titles > a");
//
//     let teams_tag_selector = Selector::parse("h2");
//     let mut team_index: u8 = 0;
//
//     match teams_selector {
//         Ok(teams_selector) => match teams_tag_selector {
//             Ok(teams_tag_selector) => {
//                 for team in match_document.select(&teams_selector) {
//                     if let Some(team_link) = team.value().attr("href") {
//                         for team_tags in team.select(&teams_tag_selector) {
//                             if team_tags
//                                 .text()
//                                 .collect::<Vec<_>>()
//                                 .join(" ")
//                                 .to_uppercase()
//                                 == TEAM
//                             {
//                                 break;
//                             }
//                             team_index += 1;
//                         }
//
//                         // let team_request = reqwest::get(team_link).await;
//                         let team_request = reqwest::blocking::get(team_link);
//
//                         match team_request {
//                             Ok(team_request) => {
//                                 let team_request_text = team_request.text();
//                                 match team_request_text {
//                                     Ok(team_request_text) => {
//                                         let team_document =
//                                             Html::parse_document(&team_request_text);
//                                         println!("{:?}", team_document);
//                                     }
//                                     Err(e) => {
//                                         eprintln!("{:?}", e);
//                                         return Err(());
//                                     }
//                                 }
//                             }
//                             Err(e) => {
//                                 eprintln!("{:?}", e);
//                                 return Err(());
//                             }
//                         }
//                     }
//                 }
//             }
//             Err(e) => {
//                 eprintln!("{:?}", e);
//                 return Err(());
//             }
//         },
//         Err(e) => {
//             eprintln!("{:?}", e);
//             return Err(());
//         }
//     }
//
//     return Ok(Teams { data: vec![] });
// }
