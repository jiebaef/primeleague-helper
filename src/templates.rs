use askama::Template;

// pub struct Container {
//     pub id: String,
//     pub description: String,
// }

// MODELS ^^^ |||| TEMPLATES VVV

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

// #[derive(Template)]
// #[template(path = "containers.html")]
// pub struct Containers {
//     pub containers: Vec<Container>,
// }
