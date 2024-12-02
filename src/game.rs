use crate::{Map, Player};

pub enum GameState {
    Running,
    Paused,
    GameOver,
}

pub struct Game {
    pub player: Player,
    pub state: GameState,
    pub map: Map,
}

impl Game {
    pub fn new(player_name: String, map: Map) -> Self {
        Game {
            player: Player::new(player_name),
            state: GameState::Running,
            map,
        }
    }

    pub fn update(&mut self) {}
}
