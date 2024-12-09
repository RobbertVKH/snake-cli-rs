use crate::direction::Direction;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]

/// Represents a point in a 2D grid.
///
/// The `Point` struct defines a coordinate with `x` and `y` values, 
/// providing functionality to translate the point in a specified direction.
///
/// # Fields
/// - `x`: The horizontal position of the point.
/// - `y`: The vertical position of the point.
///
/// # Methods
/// ## `new`
/// Creates a new `Point` with the specified coordinates.
///
/// ### Parameters
/// - `x`: The horizontal coordinate.
/// - `y`: The vertical coordinate.
///
/// ### Returns
/// A new instance of `Point`.
///
/// ### Example
/// ```rust
/// let point = Point::new(5, 10);
/// assert_eq!(point.x, 5);
/// assert_eq!(point.y, 10);
/// ```
///
/// ## `transform`
/// Translates the current point by applying a specified transformation in a given direction multiple times.
///
/// ### Parameters
/// - `direction`: The `Direction` in which to translate the point.
/// - `times`: The number of steps to move in the specified direction.
///
/// ### Returns
/// A new `Point` representing the transformed position.
///
/// ### Example
/// ```rust
/// let start = Point::new(5, 5);
/// let moved = start.transform(Direction::Up, 2);
/// assert_eq!(moved, Point::new(5, 3));
/// ```
///
/// ## `transform_value`
/// A private helper method to apply a signed transformation to a single coordinate value.
///
/// - Ensures that the result is non-negative.
/// - Panics if the transformation would result in a negative value.
///
/// ### Parameters
/// - `value`: The original coordinate value.
/// - `by`: The signed amount to transform the value.
///
/// ### Returns
/// The transformed coordinate as a `u16`.
///
/// ### Panics
/// Panics if the transformation would result in a negative coordinate.
///
/// ### Example
/// ```rust
/// let result = Point::transform_value(10, -5);
/// assert_eq!(result, 5);
/// ```

pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {

    pub fn new(x: u16, y: u16) -> Self {
        Self { x , y }
    }

    pub fn transform(&self, direction: Direction, times: u16) ->  Self {

        let times = times as i16;
        let transformation = match direction {
            Direction::Up => (0, -times),
            Direction::Right => (times, 0),
            Direction::Down => (0, times),
            Direction::Left => (-times, 0),
        };
    
        Self::new(
            Self::transform_value(self.x, transformation.0),
            Self::transform_value(self.y, transformation.1)
        )
    }

    fn transform_value(value: u16, by: i16) -> u16 {
        if by.is_negative() && by.abs() as u16 > value {
            panic!("Transforming value {} by {} would result in a negative number", value, by);
        } else {
            (value as i16 + by) as u16
        }
    }
}