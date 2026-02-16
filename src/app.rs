use crate::audio::AudioSystem;
use crate::config::Config;
use crate::input_handler::{AppState, InputHandler};
use crate::game_state::GameState;
use crate::input::{button_to_input, keycode_to_input};
use crate::leaderboard_state::LeaderboardState;
use crate::menu_state::MenuState;
use crate::persistence::Leaderboard;
use crate::rendering::{GameRenderer, MenuRenderer};
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, GameControllerSubsystem, Sdl};
use std::time::{Duration, Instant};

pub struct App {
    // SDL context
    _sdl_context: Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    _game_controller_subsystem: GameControllerSubsystem,

    // Game systems
    audio: AudioSystem,
    menu_renderer: MenuRenderer,
    game_renderer: GameRenderer,

    // Game state
    app_state: AppState,
    menu_state: MenuState,
    game_state: GameState,
    leaderboard_state: LeaderboardState,

    // Timing
    last_update: Instant,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize SDL2
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(
                "Neon Snake",
                config.visual.window_width,
                config.visual.window_height,
            )
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().build()?;
        let event_pump = sdl_context.event_pump()?;

        // Initialize game controller subsystem
        let game_controller_subsystem = sdl_context.game_controller()?;
        let _controller = if game_controller_subsystem.num_joysticks()? > 0 {
            game_controller_subsystem.open(0).ok()
        } else {
            None
        };

        // Initialize systems
        let audio = AudioSystem::new()?;
        let menu_renderer = MenuRenderer::new(
            config.visual.window_width,
            config.visual.window_height,
        )?;
        let game_renderer = GameRenderer::new(&config)?;

        // Initialize states
        let app_state = AppState::MainMenu;
        let menu_state = MenuState::new();
        let game_state = GameState::new(config.clone());
        let leaderboard_state = LeaderboardState::new();

        let last_update = Instant::now();

        Ok(App {
            _sdl_context: sdl_context,
            canvas,
            event_pump,
            _game_controller_subsystem: game_controller_subsystem,
            audio,
            menu_renderer,
            game_renderer,
            app_state,
            menu_state,
            game_state,
            leaderboard_state,
            last_update,
        })
    }

    pub fn run(&mut self) {
        'running: loop {
            // Handle events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,

                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => {
                        if let Some(input) = keycode_to_input(keycode) {
                            InputHandler::handle_input(
                                &mut self.app_state,
                                &mut self.menu_state,
                                &mut self.game_state,
                                &mut self.leaderboard_state,
                                input,
                                &self.audio,
                            );
                        }
                    }

                    Event::ControllerButtonDown { button, .. } => {
                        if let Some(input) = button_to_input(button) {
                            InputHandler::handle_input(
                                &mut self.app_state,
                                &mut self.menu_state,
                                &mut self.game_state,
                                &mut self.leaderboard_state,
                                input,
                                &self.audio,
                            );
                        }
                    }

                    _ => {}
                }
            }

            // Update game logic
            if self.app_state == AppState::Playing {
                let now = Instant::now();
                let elapsed = now.duration_since(self.last_update);

                // Update interpolation progress for smooth movement
                self.game_state.update_interpolation(elapsed.as_millis() as u64);

                if elapsed >= Duration::from_millis(self.game_state.current_speed_ms as u64) {
                    let food_eaten = self.game_state.update();
                    if food_eaten {
                        self.audio.play_crunch();
                    }

                    if self.game_state.game_over {
                        // Check if it's a high score
                        let mut leaderboard = Leaderboard::load();
                        if leaderboard.is_high_score(self.game_state.score) {
                            leaderboard.add_score("PLAYER".to_string(), self.game_state.score);
                            leaderboard.save().ok();
                        }
                    }

                    self.last_update = now;
                }
            }

            // Render
            match self.app_state {
                AppState::MainMenu => self.menu_renderer.render(&mut self.canvas, &self.menu_state),
                AppState::Playing => self.game_renderer.render_game(&mut self.canvas, &self.game_state),
                AppState::Leaderboard => {
                    self.game_renderer
                        .render_leaderboard(&mut self.canvas, &self.leaderboard_state)
                }
            }

            // Frame rate limiting
            std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
        }
    }
}
