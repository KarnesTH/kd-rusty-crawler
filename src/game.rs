//! Game module containing core game logic and state management.

use crate::{Map, Player};

/// Represents the current state of the game.
#[derive(Debug)]
pub enum GameState {
    /// Game is actively running
    Running,
    /// Game is temporarily paused
    Paused,
    /// Game has ended
    GameOver,
}

/// Main game structure holding all game-related data.
#[derive(Debug)]
pub struct Game {
    /// The player character
    pub player: Player,
    /// Current state of the game
    pub state: GameState,
    /// Game world map
    pub map: Map,
}

impl Game {
    /// Creates a new game instance.
    ///
    /// # Arguments
    /// * `player_name` - Name of the player character
    /// * `map` - Initial game map
    ///
    /// # Returns
    /// A new Game instance with the specified player and map
    pub fn new(player_name: String, map: Map) -> Self {
        Game {
            player: Player::new(player_name),
            state: GameState::Running,
            map,
        }
    }

    /// Updates the game state.
    ///
    /// Currently empty, will be implemented with game logic.
    pub fn update(&mut self) {}
}
