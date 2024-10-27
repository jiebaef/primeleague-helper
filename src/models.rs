#[derive(Debug)]
pub(crate) struct Player {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub(crate) struct Team {
    pub data: Vec<Player>,
}
