use sdl2::keyboard::Keycode;
use sdl2::controller::Button;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameInput {
    Up,
    Down,
    Left,
    Right,
    Select,
    Pause,
    Back,
    Quit,
}

pub fn keycode_to_input(keycode: Keycode) -> Option<GameInput> {
    match keycode {
        Keycode::Up | Keycode::W => Some(GameInput::Up),
        Keycode::Down | Keycode::S => Some(GameInput::Down),
        Keycode::Left | Keycode::A => Some(GameInput::Left),
        Keycode::Right | Keycode::D => Some(GameInput::Right),
        Keycode::Return | Keycode::Space => Some(GameInput::Select),
        Keycode::Escape => Some(GameInput::Back),
        _ => None,
    }
}

pub fn button_to_input(button: Button) -> Option<GameInput> {
    match button {
        Button::DPadUp => Some(GameInput::Up),
        Button::DPadDown => Some(GameInput::Down),
        Button::DPadLeft => Some(GameInput::Left),
        Button::DPadRight => Some(GameInput::Right),
        Button::A => Some(GameInput::Select),
        Button::B => Some(GameInput::Back),
        Button::Start => Some(GameInput::Pause),
        _ => None,
    }
}
