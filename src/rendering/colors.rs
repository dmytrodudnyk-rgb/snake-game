use sdl2::pixels::Color;

// Neon color palette
pub const BACKGROUND: Color = Color::RGB(10, 10, 20);
pub const GRID: Color = Color::RGBA(26, 26, 46, 38); // ~0.15 alpha
pub const SNAKE: Color = Color::RGB(0, 255, 249); // Electric cyan
pub const FOOD: Color = Color::RGB(255, 0, 110); // Hot pink
pub const TEXT: Color = Color::RGB(57, 255, 20); // Neon green
pub const TEXT_DIM: Color = Color::RGB(80, 80, 80); // Dim gray
pub const HIGHLIGHT: Color = Color::RGB(0, 255, 249); // Cyan highlight
