

pub enum HomepageEvent{
    NewPlayer(String),
}

pub struct HomepageState{
    pub nb_player: usize,
    pub last_players: Vec<String>,
}