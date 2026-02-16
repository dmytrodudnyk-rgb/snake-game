use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Placeholder text renderer using colored rectangles
/// TODO: Replace with SDL2_ttf in Phase 2
pub struct TextRenderer {
    window_width: u32,
}

impl TextRenderer {
    pub fn new(window_width: u32) -> Self {
        TextRenderer { window_width }
    }

    pub fn draw_text(
        &self,
        canvas: &mut Canvas<Window>,
        text: &str,
        x: u32,
        y: u32,
        color: Color,
    ) {
        canvas.set_draw_color(color);
        // Placeholder: draw rectangle representing text
        canvas
            .fill_rect(Rect::new(
                x as i32,
                y as i32,
                (text.len() as u32 * 8).min(self.window_width - x),
                20,
            ))
            .ok();
    }

    pub fn draw_text_centered(
        &self,
        canvas: &mut Canvas<Window>,
        text: &str,
        x: u32,
        y: u32,
        color: Color,
    ) {
        let text_width = text.len() as u32 * 8; // Rough estimate
        let text_x = x.saturating_sub(text_width / 2);
        self.draw_text(canvas, text, text_x, y, color);
    }
}
