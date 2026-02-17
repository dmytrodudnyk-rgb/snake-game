use crate::menu_state::{MenuItem, MenuState};
use crate::rendering::{colors, text_renderer::TextRenderer};
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

pub struct MenuRenderer {
    text_renderer: TextRenderer,
    window_width: u32,
    window_height: u32,
}

impl MenuRenderer {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let text_renderer = TextRenderer::new();
        MenuRenderer {
            text_renderer,
            window_width,
            window_height,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, font: &Font, menu: &MenuState) {
        canvas.set_draw_color(colors::BACKGROUND);
        canvas.clear();

        // Title
        self.text_renderer.draw_text_centered(
            canvas,
            font,
            "NEON SNAKE",
            (self.window_width / 2) as i32,
            (self.window_height / 4) as i32,
            colors::TEXT,
        ).ok();

        // Menu items
        let items = MenuState::get_items();
        let start_y = self.window_height / 2;
        let spacing = 50;

        for (i, item) in items.iter().enumerate() {
            let y = start_y + (i as u32 * spacing);
            let is_selected = *item == menu.selected_item;
            let text = match item {
                MenuItem::Start => "START",
                MenuItem::Leaderboard => "LEADERBOARD",
                MenuItem::Exit => "EXIT",
            };

            let color = if is_selected {
                colors::TEXT
            } else {
                colors::TEXT_DIM
            };

            let display_text = if is_selected {
                format!("> {} <", text)
            } else {
                text.to_string()
            };

            self.text_renderer.draw_text_centered(
                canvas,
                font,
                &display_text,
                (self.window_width / 2) as i32,
                y as i32,
                color,
            ).ok();
        }

        // Controls hint
        self.text_renderer.draw_text_centered(
            canvas,
            font,
            "[Arrow Keys / D-Pad] Navigate | [Enter / A] Select | [ESC] Exit",
            (self.window_width / 2) as i32,
            (self.window_height - 50) as i32,
            colors::TEXT_DIM,
        ).ok();

        canvas.present();
    }
}
