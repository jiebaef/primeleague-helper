use crate::models::team::Team;

use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct Error {
    pub err: String,
}
