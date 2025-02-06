// use crate::models::team::Team;

use askama::Template;

#[derive(Template)]
#[template(path = "new_teams.html")]
pub struct NewTeams {}
