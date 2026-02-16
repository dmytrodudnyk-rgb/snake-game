use crate::menu_state::{MenuItem, MenuState};
use crate::rendering::{colors, text_renderer::TextRenderer};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct MenuRenderer {
    text_renderer: TextRenderer,
    window_width: u32,
    window_height: u32,
}

impl MenuRenderer {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        MenuRenderer {
            text_renderer: TextRenderer::new(window_width),
            window_width,
            window_height,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, menu: &MenuState) {
        canvas.set_draw_color(colors::BACKGROUND);
        canvas.clear();

        // Title
        self.text_renderer.draw_text_centered(
            canvas,
            "NEON SNAKE",
            self.window_width / 2,
            self.window_height / 4,
            colors::TEXT,
        );

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
                &display_text,
                self.window_width / 2,
                y,
                color,
            );
        }

        // Controls hint
        self.text_renderer.draw_text_centered(
            canvas,
            "[Arrow Keys / D-Pad] Navigate | [Enter / A] Select | [ESC] Exit",
            self.window_width / 2,
            self.window_height - 50,
            colors::TEXT_DIM,
        );

        canvas.present();
    }
}
