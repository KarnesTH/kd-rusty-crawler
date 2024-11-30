use crate::Player;

pub enum GameState {
    Running,
    Paused,
    GameOver,
}

pub struct Game {
    pub player: Player,
    pub state: GameState,
}

impl Game {
    pub fn new(player_name: String) -> Self {
        Game {
            player: Player::new(player_name),
            state: GameState::Running,
        }
    }

    pub fn update(&mut self) {}
}
