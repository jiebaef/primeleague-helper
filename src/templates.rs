use crate::models::Team;
use askama::Template;

// MODELS ^^^ |||| TEMPLATES VVV

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "teams.html")]
pub struct Teams {
    pub data: Vec<Team>,
}

#[derive(Template)]
#[template(path = "split.html")]
pub struct Split {
    pub split: String,
}

// pub struct MatchDetails {
//     pub split: String,
//     pub teams: Teams,
// }
