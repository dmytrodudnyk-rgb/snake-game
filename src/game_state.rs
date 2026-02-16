use crate::config::Config;
use rand::RngExt;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct GameState {
    pub snake: VecDeque<Position>,
    pub direction: Direction,
    pub next_direction: Option<Direction>, // Input buffer
    pub food: Position,
    pub score: u32,
    pub game_over: bool,
    pub paused: bool,
    pub current_speed_ms: u32,
    pub interpolation_progress: f32, // 0.0 to 1.0 for smooth movement animation
    grid_size: u32,
    config: Config,
}

impl GameState {
    pub fn new(config: Config) -> Self {
        let grid_size = config.gameplay.grid_size;
        let center = (grid_size / 2) as i32;

        let mut snake = VecDeque::new();
        snake.push_back(Position { x: center, y: center });
        snake.push_back(Position { x: center - 1, y: center });
        snake.push_back(Position { x: center - 2, y: center });

        let mut game = GameState {
            snake,
            direction: Direction::Right,
            next_direction: None,
            food: Position { x: 0, y: 0 },
            score: 0,
            game_over: false,
            paused: false,
            current_speed_ms: config.gameplay.initial_speed_ms,
            interpolation_progress: 0.0,
            grid_size,
            config,
        };

        game.spawn_food();
        game
    }

    pub fn reset(&mut self) {
        let center = (self.grid_size / 2) as i32;

        self.snake.clear();
        self.snake.push_back(Position { x: center, y: center });
        self.snake.push_back(Position { x: center - 1, y: center });
        self.snake.push_back(Position { x: center - 2, y: center });

        self.direction = Direction::Right;
        self.next_direction = None;
        self.score = 0;
        self.game_over = false;
        self.paused = false;
        self.current_speed_ms = self.config.gameplay.initial_speed_ms;
        self.interpolation_progress = 0.0;
        self.spawn_food();
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        // Prevent reversing into self
        if new_direction != self.direction.opposite() {
            // Input buffering: store next direction if different from current
            if new_direction != self.direction {
                self.next_direction = Some(new_direction);
            }
        }
    }

    pub fn update(&mut self) -> bool {
        if self.game_over || self.paused {
            return false;
        }

        // Reset interpolation for new movement step
        self.interpolation_progress = 0.0;

        // Apply buffered input
        if let Some(next_dir) = self.next_direction.take() {
            if next_dir != self.direction.opposite() {
                self.direction = next_dir;
            }
        }

        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position { x: head.x, y: head.y - 1 },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x - 1, y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        };

        // Check wall collision
        if new_head.x < 0 || new_head.x >= self.grid_size as i32
            || new_head.y < 0 || new_head.y >= self.grid_size as i32 {
            self.game_over = true;
            return false;
        }

        // Check self collision
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return false;
        }

        self.snake.push_front(new_head);

        // Check food collision
        if new_head == self.food {
            self.score += 10;
            self.spawn_food();
            self.increase_speed();
            return true; // Food eaten
        } else {
            self.snake.pop_back();
            return false;
        }
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::rng();
        loop {
            let pos = Position {
                x: rng.random_range(0..self.grid_size as i32),
                y: rng.random_range(0..self.grid_size as i32),
            };
            if !self.snake.contains(&pos) {
                self.food = pos;
                break;
            }
        }
    }

    fn increase_speed(&mut self) {
        let decrease = self.config.gameplay.speed_increase_per_food;
        let min_speed = self.config.gameplay.min_speed_ms;

        if self.current_speed_ms > min_speed {
            self.current_speed_ms = self.current_speed_ms.saturating_sub(decrease).max(min_speed);
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Update interpolation progress for smooth movement animation
    pub fn update_interpolation(&mut self, elapsed_ms: u64) {
        if !self.game_over && !self.paused {
            self.interpolation_progress = (elapsed_ms as f32 / self.current_speed_ms as f32).min(1.0);
        }
    }

    /// Get interpolated head position for smooth rendering
    pub fn get_interpolated_head(&self) -> (f32, f32) {
        if self.snake.is_empty() {
            return (0.0, 0.0);
        }

        let current_head = self.snake.front().unwrap();

        // Calculate previous head position based on direction
        let (prev_x, prev_y) = match self.direction {
            Direction::Up => (current_head.x, current_head.y + 1),
            Direction::Down => (current_head.x, current_head.y - 1),
            Direction::Left => (current_head.x + 1, current_head.y),
            Direction::Right => (current_head.x - 1, current_head.y),
        };

        // Linear interpolation
        let interp_x = prev_x as f32 + (current_head.x - prev_x) as f32 * self.interpolation_progress;
        let interp_y = prev_y as f32 + (current_head.y - prev_y) as f32 * self.interpolation_progress;

        (interp_x, interp_y)
    }
}
