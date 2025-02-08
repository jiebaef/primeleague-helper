// use crate::models::team::Team;

use askama::Template;

#[derive(Template)]
#[template(path = "new_teams.html")]
pub struct NewTeams {
    pub tag: String,
    pub matches: Vec<PlMatch>,
}

#[derive(Debug)]
pub struct PlMatch {
    pub val: String,
}
