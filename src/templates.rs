use askama::Template;

// pub struct Container {
//     pub id: String,
//     pub description: String,
// }
#[derive(Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
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

// #[derive(Template)]
// #[template(path = "containers.html")]
// pub struct Containers {
//     pub containers: Vec<Container>,
// }
