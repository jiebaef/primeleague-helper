use crate::models::team::Team;

use askama::Template;

#[derive(Template)]
#[template(path = "teams.html")]
pub struct Teams {
    pub data: Vec<Team>,
}
