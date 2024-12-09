// rust requires explicit module definitions through use of "mod", though only used in game.rs
mod snake;
mod direction;
mod game;
mod points;
mod command;

use crate::game::Game;
use std::io::stdout;

/// Main entry point for the Snake game.
///
/// This file contains the necessary module imports and initialization for the Snake game.
/// It starts the game with a terminal UI size of 30 columns and 10 rows.
///
/// # Modules
/// - `snake`: Contains the logic for the Snake's movement, growth, and collision detection.
/// - `direction`: Defines the `Direction` enum representing the four movement directions.
/// - `game`: Manages the game state, including the snake, food, and game loop.
/// - `points`: Defines the `Point` struct, representing coordinates on the grid.
/// - `command`: Contains the `Command` enum for handling user input.
///
/// # Execution
/// The `main` function initializes a new game and runs it with the specified terminal dimensions (width: 30, height: 10).
///
/// # Example
/// ```rust
/// // Start a new game with a 30x10 terminal UI
/// Game::new(stdout(), 30, 10).run();
/// ```
fn main() {
    Game::new(stdout(), 30, 10).run(); // stdout, height and width of terminal ui
}
