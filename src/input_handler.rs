use crate::audio::AudioSystem;
use crate::game_state::{Direction, GameState};
use crate::input::GameInput;
use crate::leaderboard_state::LeaderboardState;
use crate::menu_state::{MenuItem, MenuState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    MainMenu,
    Playing,
    Leaderboard,
}

pub struct InputHandler;

impl InputHandler {
    pub fn handle_input(
        app_state: &mut AppState,
        menu_state: &mut MenuState,
        game_state: &mut GameState,
        leaderboard_state: &mut LeaderboardState,
        input: GameInput,
        audio: &AudioSystem,
    ) {
        match *app_state {
            AppState::MainMenu => {
                Self::handle_menu_input(app_state, menu_state, game_state, leaderboard_state, input, audio);
            }
            AppState::Playing => {
                Self::handle_game_input(app_state, game_state, input);
            }
            AppState::Leaderboard => {
                Self::handle_leaderboard_input(app_state, input);
            }
        }
    }

    fn handle_menu_input(
        app_state: &mut AppState,
        menu_state: &mut MenuState,
        game_state: &mut GameState,
        leaderboard_state: &mut LeaderboardState,
        input: GameInput,
        audio: &AudioSystem,
    ) {
        match input {
            GameInput::Up => {
                menu_state.move_up();
                audio.play_click();
            }
            GameInput::Down => {
                menu_state.move_down();
                audio.play_click();
            }
            GameInput::Select => {
                audio.play_click();
                match menu_state.selected_item {
                    MenuItem::Start => {
                        game_state.reset();
                        *app_state = AppState::Playing;
                    }
                    MenuItem::Leaderboard => {
                        leaderboard_state.refresh();
                        *app_state = AppState::Leaderboard;
                    }
                    MenuItem::Exit => {
                        std::process::exit(0);
                    }
                }
            }
            GameInput::Back => {
                std::process::exit(0);
            }
            _ => {}
        }
    }

    fn handle_game_input(app_state: &mut AppState, game_state: &mut GameState, input: GameInput) {
        if game_state.game_over {
            match input {
                GameInput::Back => {
                    *app_state = AppState::MainMenu;
                }
                GameInput::Select => {
                    game_state.reset();
                }
                _ => {}
            }
        } else {
            match input {
                GameInput::Up => game_state.set_direction(Direction::Up),
                GameInput::Down => game_state.set_direction(Direction::Down),
                GameInput::Left => game_state.set_direction(Direction::Left),
                GameInput::Right => game_state.set_direction(Direction::Right),
                GameInput::Back => {
                    game_state.toggle_pause();
                }
                _ => {}
            }
        }
    }

    fn handle_leaderboard_input(app_state: &mut AppState, input: GameInput) {
        if input == GameInput::Back {
            *app_state = AppState::MainMenu;
        }
    }
}
