use once_cell::sync::OnceCell;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::Path;

/// Global TTF context singleton
static TTF_CONTEXT: OnceCell<Sdl2TtfContext> = OnceCell::new();

/// Initialize the global TTF context (must be called once at startup)
pub fn init_ttf() -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| format!("SDL2_ttf init failed: {}", e))?;
    TTF_CONTEXT
        .set(ttf_context)
        .map_err(|_| "TTF context already initialized".to_string())?;
    Ok(())
}

/// Get a reference to the global TTF context
pub fn get_ttf_context() -> &'static Sdl2TtfContext {
    TTF_CONTEXT
        .get()
        .expect("TTF context not initialized - call init_ttf() first")
}

/// Load a font from the global TTF context
pub fn load_font(font_path: &Path, point_size: u16) -> Result<Font<'static, 'static>, String> {
    let ttf_context = get_ttf_context();
    ttf_context
        .load_font(font_path, point_size)
        .map_err(|e| format!("Failed to load font: {}", e))
}

/// Load the main game font (Press Start 2P)
pub fn load_main_font(point_size: u16) -> Result<Font<'static, 'static>, String> {
    let font_path = Path::new("third-party/fonts/PressStart2P.ttf");
    load_font(font_path, point_size)
}
