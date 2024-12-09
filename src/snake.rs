use crate::direction::Direction;
use crate::points::Point;

#[derive(Debug)]

/// Represents the snake in the Snake game.
///
/// The `Snake` struct maintains the state and behavior of the snake,
/// including its body, direction, and growth status.
///
/// # Fields
/// - `body`: A vector of `Point` representing the positions of the snake's segments.
/// - `direction`: The current direction of the snake's movement.
/// - `digesting`: Indicates whether the snake is in a growth state (e.g., after eating food).
///
/// # Methods
/// ## `new`
/// Creates a new `Snake` with a specified starting point, length, and direction.
///
/// ### Parameters
/// - `start`: The `Point` where the snake's head starts.
/// - `length`: The initial length of the snake.
/// - `direction`: The `Direction` the snake is facing.
///
/// ### Returns
/// A new instance of the `Snake`.
///
/// ### Example
/// ```rust
/// let start = Point::new(5, 5);
/// let snake = Snake::new(start, 3, Direction::Right);
/// ```
///
/// ## `get_head_point`
/// Returns the position of the snake's head.
///
/// ### Returns
/// A `Point` representing the head's position.
///
/// ## `get_body_points`
/// Returns a vector of points representing the snake's body.
///
/// ### Returns
/// A `Vec<Point>` containing all body segment positions.
///
/// ## `get_direction`
/// Returns the current direction of the snake.
///
/// ### Returns
/// A `Direction` indicating the snake's movement direction.
///
/// ## `contains_point`
/// Checks if the snake's body contains a specific point.
///
/// ### Parameters
/// - `point`: A reference to a `Point` to check.
///
/// ### Returns
/// `true` if the point is part of the snake's body, otherwise `false`.
///
/// ## `slither`
/// Moves the snake forward by one step in its current direction.
///
/// - If `digesting` is `true`, the snake grows and does not remove its last segment.
/// - If `digesting` is `false`, the snake moves normally, and its tail segment is removed.
///
/// ## `set_direction`
/// Updates the snake's direction.
///
/// ### Parameters
/// - `direction`: The new `Direction` for the snake.
///
/// ## `grow`
/// Marks the snake for growth, adding an additional segment after its next move.

pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {

    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();

        let body: Vec<Point> = (0..length)
        .into_iter()  
        .map(|i| start.transform(opposite, i))
        .collect();

        Self { body, direction, digesting: false }
    }

    pub fn get_head_point(&self) -> Point {
        self.body.first().unwrap().clone() 
    }

    pub fn get_body_points(&self) -> Vec<Point> {
        self.body.clone()
    }

    pub fn get_direction(&self) -> Direction { 
        self.direction
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.body.contains(point)
    }

    pub fn slither(&mut self) {

        self.body.insert(0, self.body.first().unwrap().transform(self.direction, 1)); 

        // if digesting is true, we don't remove the newly added block
        if !self.digesting {
            self.body.remove(self.body.len() - 1);
        }
        else {
            self.digesting = false;
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn grow(&mut self) {
        self.digesting = true;
    }
}