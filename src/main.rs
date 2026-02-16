mod app;
mod audio;
mod config;
mod game_state;
mod input;
mod input_handler;
mod leaderboard_state;
mod menu_state;
mod persistence;
mod rendering;
mod resources;

use app::App;
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize TTF subsystem
    resources::init_ttf()?;

    let config = Config::load()?;
    let mut app = App::new(config)?;
    app.run();
    Ok(())
}
