use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Player {
    pub pseudo: String,
    pub is_bot: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum HomepageEvent {
    NewPlayer(Player),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HomepageState {
    pub nb_player: usize,
    pub last_players: Vec<Player>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum HomepageSse {
    Event(HomepageEvent),
    State(HomepageState),
}
