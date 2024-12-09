#[derive(Debug, Copy, Clone, Eq, PartialEq)]

/// Represents the possible directions for movement.
///
/// The `Direction` enum defines the four cardinal directions (Up, Right, Down, Left) that can be used 
/// in the game to control the movement of the snake.
///
/// # Variants
/// - `Up`: Represents the upward direction.
/// - `Right`: Represents the rightward direction.
/// - `Down`: Represents the downward direction.
/// - `Left`: Represents the leftward direction.
///
/// # Methods
/// ## `opposite`
/// Returns the opposite direction of the current direction.
/// For example, `Up`'s opposite is `Down`, and `Right`'s opposite is `Left`.
///
/// # Example
/// ```rust
/// use crate::direction::Direction;
///
/// let up = Direction::Up;
/// let down = up.opposite(); // Direction::Down
/// ```
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right
        }
    }
}