use crate::models::Team;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "teams.html")]
pub struct Teams {
    pub data: Vec<Team>,
}

// pub struct MatchDetails {
//     pub split: String,
//     pub teams: Teams,
// }
