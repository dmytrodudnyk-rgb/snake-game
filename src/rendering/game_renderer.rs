use crate::config::Config;
use crate::game_state::{GameState, Position};
use crate::leaderboard_state::LeaderboardState;
use crate::rendering::{colors, text_renderer::TextRenderer};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct GameRenderer {
    text_renderer: TextRenderer,
    cell_size: u32,
    grid_size: u32,
    window_width: u32,
    window_height: u32,
}

impl GameRenderer {
    pub fn new(config: &Config) -> Result<Self, String> {
        let cell_size = config.visual.window_width / config.gameplay.grid_size;
        let text_renderer = TextRenderer::new(16)?;
        Ok(GameRenderer {
            text_renderer,
            cell_size,
            grid_size: config.gameplay.grid_size,
            window_width: config.visual.window_width,
            window_height: config.visual.window_height,
        })
    }

    pub fn render_game(&self, canvas: &mut Canvas<Window>, game: &GameState) {
        canvas.set_draw_color(colors::BACKGROUND);
        canvas.clear();

        // Draw grid lines
        self.draw_grid(canvas);

        // Draw food
        self.draw_cell(canvas, &game.food, colors::FOOD);

        // Draw snake
        for segment in game.snake.iter() {
            self.draw_cell(canvas, segment, colors::SNAKE);
        }

        // Draw score
        self.text_renderer.draw_text(
            canvas,
            &format!("Score: {}", game.score),
            10,
            10,
            colors::TEXT,
        ).ok();

        if game.paused {
            self.text_renderer.draw_text_centered(
                canvas,
                "PAUSED",
                (self.window_width / 2) as i32,
                (self.window_height / 2) as i32,
                colors::TEXT,
            ).ok();
            self.text_renderer.draw_text_centered(
                canvas,
                "[ESC] Resume",
                (self.window_width / 2) as i32,
                (self.window_height / 2 + 40) as i32,
                colors::TEXT_DIM,
            ).ok();
        }

        if game.game_over {
            self.text_renderer.draw_text_centered(
                canvas,
                "GAME OVER",
                (self.window_width / 2) as i32,
                (self.window_height / 2 - 40) as i32,
                colors::TEXT,
            ).ok();
            self.text_renderer.draw_text_centered(
                canvas,
                &format!("Final Score: {}", game.score),
                (self.window_width / 2) as i32,
                (self.window_height / 2) as i32,
                colors::TEXT,
            ).ok();
            self.text_renderer.draw_text_centered(
                canvas,
                "[ESC] Menu | [Enter] Restart",
                (self.window_width / 2) as i32,
                (self.window_height / 2 + 40) as i32,
                colors::TEXT_DIM,
            ).ok();
        }

        canvas.present();
    }

    pub fn render_leaderboard(&self, canvas: &mut Canvas<Window>, state: &LeaderboardState) {
        canvas.set_draw_color(colors::BACKGROUND);
        canvas.clear();

        // Title
        self.text_renderer.draw_text_centered(
            canvas,
            "LEADERBOARD",
            (self.window_width / 2) as i32,
            100,
            colors::TEXT,
        ).ok();

        // Entries
        let start_y = 200;
        let spacing = 50;

        if state.leaderboard.entries.is_empty() {
            self.text_renderer.draw_text_centered(
                canvas,
                "No scores yet!",
                (self.window_width / 2) as i32,
                (start_y + 100) as i32,
                colors::TEXT_DIM,
            ).ok();
        } else {
            for (i, entry) in state.leaderboard.entries.iter().enumerate() {
                let y = start_y + (i as u32 * spacing);
                let text = format!("{}. {} .......... {}", i + 1, entry.name, entry.score);
                self.text_renderer.draw_text_centered(
                    canvas,
                    &text,
                    (self.window_width / 2) as i32,
                    y as i32,
                    colors::TEXT,
                ).ok();
            }
        }

        // Controls hint
        self.text_renderer.draw_text_centered(
            canvas,
            "[ESC] Return",
            (self.window_width / 2) as i32,
            (self.window_height - 50) as i32,
            colors::TEXT_DIM,
        ).ok();

        canvas.present();
    }

    fn draw_grid(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(colors::GRID);

        // Vertical lines
        for i in 0..=self.grid_size {
            let x = (i * self.cell_size) as i32;
            canvas
                .draw_line((x, 0), (x, self.window_height as i32))
                .ok();
        }

        // Horizontal lines
        for i in 0..=self.grid_size {
            let y = (i * self.cell_size) as i32;
            canvas
                .draw_line((0, y), (self.window_width as i32, y))
                .ok();
        }
    }

    fn draw_cell(&self, canvas: &mut Canvas<Window>, pos: &Position, color: Color) {
        canvas.set_draw_color(color);
        let x = (pos.x as u32 * self.cell_size) as i32;
        let y = (pos.y as u32 * self.cell_size) as i32;
        let size = self.cell_size as u32;

        // Draw with small padding for segmented look
        let padding = 2;
        canvas
            .fill_rect(Rect::new(
                x + padding,
                y + padding,
                size - (padding * 2) as u32,
                size - (padding * 2) as u32,
            ))
            .ok();
    }
}
