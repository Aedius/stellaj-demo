use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize )]
pub enum HomepageEvent{
    NewPlayer(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize )]
pub struct HomepageState{
    pub nb_player: usize,
    pub last_players: Vec<String>,
}