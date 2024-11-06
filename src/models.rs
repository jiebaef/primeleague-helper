#[derive(Debug)]
pub(crate) struct Player {
    pub id: String,
    pub name: String,
    pub link: String,
    pub game_account: String,
}

#[derive(Debug)]
pub(crate) struct Team {
    pub name: String,
    pub data: Vec<Player>,
    pub opgg: String,
}
