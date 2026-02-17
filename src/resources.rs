// Resource loading functions - pure loaders with no state management
use sdl2::mixer::Chunk;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::Path;

// =============================================================================
// Font Resource Loading
// =============================================================================

/// Load a font file at the specified point size
pub fn load_font<'a>(
    ttf_context: &'a Sdl2TtfContext,
    font_path: &Path,
    point_size: u16,
) -> Result<Font<'a, 'a>, String> {
    ttf_context
        .load_font(font_path, point_size)
        .map_err(|e| format!("Failed to load font: {}", e))
}

/// Load the main game font (Press Start 2P)
pub fn load_main_font<'a>(ttf_context: &'a Sdl2TtfContext, point_size: u16) -> Result<Font<'a, 'a>, String> {
    let font_path = Path::new("assets/fonts/PressStart2P.ttf");
    if !font_path.exists() {
        return Err(format!("Font file not found: {:?} (cwd: {:?})", font_path, std::env::current_dir()));
    }
    load_font(ttf_context, font_path, point_size)
}

// =============================================================================
// Audio Resource Loading
// =============================================================================

/// Load the click sound effect
pub fn load_click_sound() -> Result<Chunk, String> {
    let path = "assets/sounds/click.wav";
    if !Path::new(path).exists() {
        return Err(format!("File not found: {} (cwd: {:?})", path, std::env::current_dir()));
    }
    Chunk::from_file(path)
        .map_err(|e| format!("Failed to load click.wav: {} (cwd: {:?})", e, std::env::current_dir()))
}

/// Load the crunch sound effect
pub fn load_crunch_sound() -> Result<Chunk, String> {
    let path = "assets/sounds/crunch.wav";
    if !Path::new(path).exists() {
        return Err(format!("File not found: {} (cwd: {:?})", path, std::env::current_dir()));
    }
    Chunk::from_file(path)
        .map_err(|e| format!("Failed to load crunch.wav: {} (cwd: {:?})", e, std::env::current_dir()))
}
