use crate::snake::Snake;
use crate::points::Point;
use crate::direction::Direction;

use std::io::Stdout;
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType, size, SetSize, enable_raw_mode, disable_raw_mode};
use crossterm::style::{SetForegroundColor, Print, ResetColor, Color};
use std::time::{Duration, Instant};
use crossterm::cursor::{Show, MoveTo, Hide};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers, KeyEvent};
use crate::command::Command;
use rand::Rng;

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;

#[derive(Debug)]
/// Represents the game logic and manages the state of a terminal-based Snake game.
/// 
/// The `Game` struct handles the initialization, rendering, and main game loop, including
/// interactions with the snake, food, user input, and terminal UI.
///
/// # Fields
/// - `stdout`: The standard output used to render the game's UI in the terminal.
/// - `original_terminal_size`: The terminal size before starting the game.
/// - `width`: The width of the game grid.
/// - `height`: The height of the game grid.
/// - `food`: The current position of the food on the grid (if any).
/// - `snake`: The `Snake` instance representing the player's snake.
/// - `speed`: The current speed of the game, which increases with the score.
/// - `score`: The player's current score.
///
/// # Methods
/// ## `new`
/// Creates a new instance of the `Game`.
/// 
/// ### Parameters
/// - `stdout`: The standard output used for terminal rendering.
/// - `width`: The width of the game grid.
/// - `height`: The height of the game grid.
/// 
/// ### Returns
/// A new instance of the `Game`.
///
/// ### Example
/// ```rust
/// let stdout = std::io::stdout();
/// let game = Game::new(stdout, 20, 15);
/// ```
///
/// ## `run`
/// Starts the main game loop, handling user input, rendering, and game logic.
///
/// ## `place_food`
/// Randomly places food on the grid in a location that does not overlap with the snake.
///
/// ## `render`
/// Updates the game UI, including the snake, food, and borders.
///
/// ## `prepare_ui`
/// Configures the terminal for raw mode and resizes the display for the game.
///
/// ## `calculate_interval`
/// Calculates the delay between game updates based on the current speed.
///
/// ### Returns
/// A `Duration` indicating the update interval.
///
/// ## `get_command`
/// Waits for and processes user input to return a game command.
/// 
/// ### Parameters
/// - `wait_for`: The duration to wait for user input.
///
/// ### Returns
/// An `Option<Command>` indicating the action to be taken (e.g., quit or change direction).
///
/// ## `has_collidated_with_wall`
/// Checks if the snake's head has collided with the wall.
///
/// ### Returns
/// `true` if the snake has collided with a wall, otherwise `false`.
///
/// ## `has_bitten_itself`
/// Checks if the snake's head has collided with its body.
///
/// ### Returns
/// `true` if the snake has bitten itself, otherwise `false`.
///
/// ## `restore_ui`
/// Restores the terminal to its original state after the game ends.
///
/// ## `draw_snake`
/// Renders the snake on the grid using color and symbols.
///
/// ## `draw_food`
/// Renders the food on the grid.
///
/// ## `draw_background`
/// Clears the grid area of the game.
///
/// ## `draw_borders`
/// Draws the borders of the game grid using symbols.
///
/// # Example
/// ```rust
/// let stdout = std::io::stdout();
/// let mut game = Game::new(stdout, 20, 15);
/// game.run();
/// ```

pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16, 
    food: Option<Point>,
    snake: Snake,
    speed: u16,
    score: u16
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size: (u16, u16) = size().unwrap();

        Self {
            stdout,
            original_terminal_size,
            width,
            height,
            food: None, // generated when game starts
            snake: Snake::new(
                Point::new(width / 2, height / 2),
                3,
                match rand::thread_rng().gen_range(0, 4) {
                    0 => Direction::Up,
                    1 => Direction::Right,
                    2 => Direction::Down,
                    3 => Direction::Left,
                    _ => unreachable!()
                },
            ),
            speed: 20,
            score: 0
        }
    }

    pub fn run(&mut self) {
        self.place_food();
        self.prepare_ui();
        self.render();

        let mut done = false;

        while !done {
            let interval = self.calculate_interval();
            let direction = self.snake.get_direction();
            let now = Instant::now();

            while now.elapsed() < interval {
                if let Some(command) = self.get_command(interval - now.elapsed()) {
                    match command {
                        Command::Quit => {
                            done = true;
                            break;
                        }
                        Command::Turn(towards) => {
                            if direction != towards && direction.opposite() != towards {
                                self.snake.set_direction(towards); 
                            }
                        }
                    }
                }
            }

            if self.has_collidated_with_wall() || self.has_bitten_itself() {
                done = true;
            } else {
                self.snake.slither();

                if let Some(food_point) = self.food {
                    if self.snake.get_head_point() == food_point {
                        self.snake.grow(); 
                        self.place_food();
                        self.score += 1;

                        if self.score % ((self.width  * self.height) / MAX_SPEED) == 0 {
                            self.speed += 1
                        }
                    }
                }

                self.render();
            }

        }

        self.restore_ui();

        println!("Game over! Your score is {}", self.score); 
    }

    fn place_food(&mut self) {
        loop {
            let random_x = rand::thread_rng().gen_range(0, self.width);
            let random_y = rand::thread_rng().gen_range(0, self.height);

            let point = Point::new(random_x, random_y);
            if !self.snake.contains_point(&point) {
                self.food = Some(point);
                break;
            }
        }
    }

    fn render(&mut self) {
        self.draw_borders();
        self.draw_background();
        self.draw_food();
        self.draw_snake();
    }

    fn prepare_ui(&mut self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Hide).unwrap();
    }

    fn calculate_interval(&self) -> Duration {
        let speed = MAX_SPEED - self.speed;
        Duration::from_millis(
            (MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64
        )
    }

    fn get_command(&self, wait_for: Duration) -> Option<Command> {
        let key_event = self.wait_for_key_event(wait_for)?;

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(Command::Quit),
            KeyCode::Char('c') | KeyCode::Char('C') =>
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Some(Command::Quit)
                } else {
                    None
                }
            KeyCode::Up => Some(Command::Turn(Direction::Up)),
            KeyCode::Right => Some(Command::Turn(Direction::Right)),
            KeyCode::Down => Some(Command::Turn(Direction::Down)),
            KeyCode::Left => Some(Command::Turn(Direction::Left)),
            _ => None
        }
    }

    fn wait_for_key_event(&self, wait_for: Duration) -> Option<KeyEvent> {
        if poll(wait_for).ok()? {
            let event = read().ok()?;
            if let Event::Key(key_event) = event {
                return Some(key_event);
            }
        }

        None
    }

    fn has_collidated_with_wall(&self) -> bool {
        let head_point = self.snake.get_head_point();

        match self.snake.get_direction() {
            Direction::Up => head_point.y == 0,
            Direction::Right => head_point.x == self.width - 1,
            Direction::Down => head_point.y == self.height - 1,
            Direction::Left => head_point.x == 0,
        }
    }

    fn has_bitten_itself(&self) -> bool {
        let next_head_point = self.snake.get_head_point().transform(self.snake.get_direction(), 1);
        let mut next_body_points = self.snake.get_body_points().clone();

        next_body_points.remove(next_body_points.len() - 1);
        next_body_points.remove(0);

        next_body_points.contains(&next_head_point)
    }

    fn restore_ui(&mut self) {
        let (cols, rows) = self.original_terminal_size;
        self.stdout
            .execute(SetSize(cols, rows)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Show).unwrap()
            .execute(ResetColor).unwrap();
            disable_raw_mode().unwrap();
    }

    fn draw_snake(&mut self) {
        let fg = SetForegroundColor(match self.speed % 3 {
            0 => Color::Green,
            1 => Color::Cyan,
            _ => Color::Yellow
        });
        self.stdout.execute(fg).unwrap();

        let body_points = self.snake.get_body_points();
        for (i, body) in body_points.iter().enumerate() {
            let previous = if i == 0 { None } else { body_points.get(i - 1) };
            let next = body_points.get(i + 1);
            let symbol = if let Some(&next) = next {
                if let Some(&previous) = previous {
                    if previous.x == next.x {
                        '║'
                    } else if previous.y == next.y {
                        '═'
                    } else {
                        let d = body.transform(Direction::Down, 1);
                        let r = body.transform(Direction::Right, 1);
                        let u = if body.y == 0 { body.clone() } else { body.transform(Direction::Up, 1) };
                        let l = if body.x == 0 { body.clone() } else { body.transform(Direction::Left, 1) };
                        if (next == d && previous == r) || (previous == d && next == r) {
                            '╔'
                        } else if (next == d && previous == l) || (previous == d && next == l) {
                            '╗'
                        } else if (next == u && previous == r) || (previous == u && next == r) {
                            '╚'
                        } else {
                            '╝'
                        }
                    }
                } else {
                    'O'
                }
            } else if let Some(&previous) = previous {
                if body.y == previous.y {
                    '═'
                } else {
                    '║'
                }
            } else {
                panic!("Invalid snake body point.");
            };

            self.stdout
                .execute(MoveTo(body.x + 1, body.y + 1)).unwrap()
                .execute(Print(symbol)).unwrap();
        }
    }

    fn draw_food(&mut self) {
        self.stdout.execute(SetForegroundColor(Color::White)).unwrap();

        for food in self.food.iter() {
            self.stdout
                .execute(MoveTo(food.x + 1, food.y + 1)).unwrap()
                .execute(Print("•")).unwrap();
        }
    }

    fn draw_background(&mut self) {
        self.stdout.execute(ResetColor).unwrap();

        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                self.stdout
                    .execute(MoveTo(x, y)).unwrap()
                    .execute(Print(" ")).unwrap();
            }
        }
    }

    fn draw_borders(&mut self) {
        self.stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();

        for y in 0..self.height + 2 {
            self.stdout
                .execute(MoveTo(0, y)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(self.width + 1, y)).unwrap()
                .execute(Print("#")).unwrap();
        }

        for x in 0..self.width + 2 {
            self.stdout
                .execute(MoveTo(x, 0)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(x, self.height + 1)).unwrap()
                .execute(Print("#")).unwrap();
        }

        self.stdout
            .execute(MoveTo(0, 0)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(self.width + 1, self.height + 1)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(self.width + 1, 0)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(0, self.height + 1)).unwrap()
            .execute(Print("#")).unwrap();
    }
}
