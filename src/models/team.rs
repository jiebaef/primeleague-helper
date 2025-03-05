use crate::models::player::Player;

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub data: Vec<Player>,
    pub opgg: String,
}
