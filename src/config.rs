use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub gameplay: GameplayConfig,
    pub visual: VisualConfig,
    pub audio: AudioConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameplayConfig {
    pub initial_speed_ms: u32,
    pub min_speed_ms: u32,
    pub speed_increase_per_food: u32,
    pub grid_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VisualConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub grid_alpha: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AudioConfig {
    pub master_volume: f32,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}
