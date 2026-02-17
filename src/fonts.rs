// Font system - manages SDL2_ttf context and fonts
use crate::resources;
use once_cell::sync::OnceCell;
use sdl2::ttf::{Font, Sdl2TtfContext};

/// Global TTF context singleton
static TTF_CONTEXT: OnceCell<Sdl2TtfContext> = OnceCell::new();

/// FontSystem - owns fonts loaded from the global TTF context
pub struct FontSystem {
    font: Font<'static, 'static>,
}

impl FontSystem {
    /// Initialize font system and load fonts
    pub fn new() -> Result<Self, String> {
        // Initialize global TTF context if not already done
        TTF_CONTEXT
            .get_or_try_init(|| {
                sdl2::ttf::init().map_err(|e| format!("SDL2_ttf init failed: {}", e))
            })?;

        // Load font from global context
        let ttf_context = TTF_CONTEXT.get().unwrap();
        let font = resources::load_main_font(ttf_context, 16)?;

        Ok(FontSystem { font })
    }

    /// Get reference to the font
    pub fn font(&self) -> &Font<'static, 'static> {
        &self.font
    }
}
