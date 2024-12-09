use crate::direction::Direction;

/// Represents a game command.
///
/// The `Command` enum defines actions that can be issued during gameplay, such as quitting the game 
/// or turning the snake in a specific direction.
///
/// # Variants
/// ## `Quit`
/// Represents a command to quit the game.
///
/// ## `Turn`
/// Represents a command to change the direction of the snake.
///
/// ### Fields
/// - `Direction`: The direction to which the snake should turn.
///
/// # Example
/// ```rust
/// use crate::direction::Direction;
/// use crate::command::Command;
///
/// let quit_command = Command::Quit;
/// let turn_command = Command::Turn(Direction::Up);
/// ```
pub enum Command {
    Quit,
    Turn(Direction)
}