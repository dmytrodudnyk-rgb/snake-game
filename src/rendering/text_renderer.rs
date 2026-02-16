use crate::resources;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

/// Text renderer using SDL2_ttf for real text rendering
pub struct TextRenderer {
    font: Font<'static, 'static>,
}

impl TextRenderer {
    /// Creates a new TextRenderer with the specified font size
    pub fn new(font_size: u16) -> Result<Self, String> {
        let font = resources::load_main_font(font_size)?;
        Ok(TextRenderer { font })
    }

    /// Draws text at the specified position
    pub fn draw_text(
        &self,
        canvas: &mut Canvas<Window>,
        text: &str,
        x: i32,
        y: i32,
        color: Color,
    ) -> Result<(), String> {
        // Render text to surface
        let surface = self
            .font
            .render(text)
            .blended(color)
            .map_err(|e| format!("Failed to render text: {}", e))?;

        // Convert surface to texture
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| format!("Failed to create texture: {}", e))?;

        // Get texture dimensions
        let query = texture.query();
        let target = Rect::new(x, y, query.width, query.height);

        // Draw texture to canvas
        canvas
            .copy(&texture, None, Some(target))
            .map_err(|e| format!("Failed to copy texture: {}", e))?;

        Ok(())
    }

    /// Draws text centered at the specified position
    pub fn draw_text_centered(
        &self,
        canvas: &mut Canvas<Window>,
        text: &str,
        x: i32,
        y: i32,
        color: Color,
    ) -> Result<(), String> {
        // Get text dimensions
        let (text_width, _) = self
            .font
            .size_of(text)
            .map_err(|e| format!("Failed to get text size: {}", e))?;

        // Calculate centered x position
        let text_x = x - (text_width as i32 / 2);

        // Draw text
        self.draw_text(canvas, text, text_x, y, color)
    }
}
