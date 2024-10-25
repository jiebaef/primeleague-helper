use askama::Template;

#[derive(Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub link: String,
}

// MODELS ^^^ |||| TEMPLATES VVV

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "players.html")]
pub struct Players {
    pub data: Vec<Player>,
}

#[derive(Template)]
#[template(path = "teams.html")]
pub struct Teams {
    pub data: Vec<Team>,
}

// #[derive(Template)]
// #[template(path = "containers.html")]
// pub struct Containers {
//     pub containers: Vec<Container>,
// }
