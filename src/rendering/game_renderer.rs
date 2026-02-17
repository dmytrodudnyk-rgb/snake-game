use crate::config::Config;
use crate::game_state::{GameState, Position};
use crate::leaderboard_state::LeaderboardState;
use crate::rendering::{colors, text_renderer::TextRenderer};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use std::time::Instant;

pub struct GameRenderer {
    text_renderer: TextRenderer,
    cell_size: u32,
    grid_size: u32,
    window_width: u32,
    window_height: u32,
    animation_start: Instant,
}

impl GameRenderer {
    pub fn new(config: &Config) -> Self {
        let cell_size = config.visual.window_width / config.gameplay.grid_size;
        let text_renderer = TextRenderer::new();
        GameRenderer {
            text_renderer,
            cell_size,
            grid_size: config.gameplay.grid_size,
            window_width: config.visual.window_width,
            window_height: config.visual.window_height,
            animation_start: Instant::now(),
        }
    }

    pub fn render_game(&self, canvas: &mut Canvas<Window>, font: &Font, game: &GameState) {
        canvas.set_draw_color(colors::BACKGROUND);
        canvas.clear();

        // Draw grid lines
        self.draw_grid(canvas);

        // Draw food with pulse animation
        self.draw_cell_pulsing(canvas, &game.food, colors::FOOD);

        // Draw snake with interpolation
        if !game.snake.is_empty() {
            // Draw body segments at grid positions (skip the head)
            for segment in game.snake.iter().skip(1) {
                self.draw_cell(canvas, segment, colors::SNAKE);
            }

            // Draw head with interpolation for smooth movement
            let (interp_x, interp_y) = game.get_interpolated_head();
            self.draw_cell_interpolated(canvas, interp_x, interp_y, colors::SNAKE);
        }

        // Draw score
        self.text_renderer.draw_text(
            canvas,
            font,
            &format!("Score: {}", game.score),
            10,
            10,
            colors::TEXT,
        ).ok();

        if game.paused {
            self.text_renderer.draw_text_centered(
                canvas,
                font,
                "PAUSED",
                (self.window_width / 2) as i32,
                (self.window_height / 2) as i32,
                colors::TEXT,
            ).ok();
            self.text_renderer.draw_text_centered(
                canvas,
                font,
                "[ESC] Resume",
                (self.window_width / 2) as i32,
                (self.window_height / 2 + 40) as i32,
                colors::TEXT_DIM,
            ).ok();
        }

        if game.game_over {
            self.text_renderer.draw_text_centered(
                canvas,
                font,
                "GAME OVER",
                (self.window_width / 2) as i32,
                (self.window_height / 2 - 40) as i32,
                colors::TEXT,
            ).ok();
            self.text_renderer.draw_text_centered(
                canvas,
                font,
                &format!("Final Score: {}", game.score),
                (self.window_width / 2) as i32,
                (self.window_height / 2) as i32,
                colors::TEXT,
            ).ok();
            self.text_renderer.draw_text_centered(
                canvas,
                font,
                "[ESC] Menu | [Enter] Restart",
                (self.window_width / 2) as i32,
                (self.window_height / 2 + 40) as i32,
                colors::TEXT_DIM,
            ).ok();
        }

        canvas.present();
    }

    pub fn render_leaderboard(&self, canvas: &mut Canvas<Window>, font: &Font, state: &LeaderboardState) {
        canvas.set_draw_color(colors::BACKGROUND);
        canvas.clear();

        // Title
        self.text_renderer.draw_text_centered(
            canvas,
            font,
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
                font,
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
                    font,
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
            font,
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

    fn draw_cell_interpolated(&self, canvas: &mut Canvas<Window>, grid_x: f32, grid_y: f32, color: Color) {
        canvas.set_draw_color(color);
        let x = (grid_x * self.cell_size as f32) as i32;
        let y = (grid_y * self.cell_size as f32) as i32;
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

    fn draw_cell_pulsing(&self, canvas: &mut Canvas<Window>, pos: &Position, color: Color) {
        canvas.set_draw_color(color);

        // Calculate pulse scale using sine wave (1 second period)
        let elapsed = self.animation_start.elapsed().as_secs_f32();
        let pulse_factor = (elapsed * std::f32::consts::TAU).sin(); // TAU = 2π
        let scale = 1.0 + (pulse_factor * 0.1); // Range: 0.9 to 1.1

        // Calculate scaled size and centered position
        let base_size = self.cell_size as f32;
        let scaled_size = (base_size * scale) as u32;
        let size_diff = (base_size - scaled_size as f32) / 2.0;

        let x = (pos.x as f32 * base_size + size_diff) as i32;
        let y = (pos.y as f32 * base_size + size_diff) as i32;

        // Draw with small padding for segmented look
        let padding = 2;
        canvas
            .fill_rect(Rect::new(
                x + padding,
                y + padding,
                scaled_size.saturating_sub((padding * 2) as u32),
                scaled_size.saturating_sub((padding * 2) as u32),
            ))
            .ok();
    }
}
