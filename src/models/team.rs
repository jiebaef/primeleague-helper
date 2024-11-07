use crate::models::player::Player;

#[derive(Debug)]
pub(crate) struct Team {
    pub name: String,
    pub data: Vec<Player>,
    pub opgg: String,
}
