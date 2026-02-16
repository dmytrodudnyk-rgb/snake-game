use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoreEntry {
    pub name: String,
    pub score: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leaderboard {
    pub entries: Vec<ScoreEntry>,
}

impl Leaderboard {
    const MAX_ENTRIES: usize = 5;
    const LEADERBOARD_FILE: &'static str = "leaderboard.json";

    pub fn load() -> Self {
        let path = Self::get_path();
        if path.exists() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(leaderboard) = serde_json::from_str(&data) {
                    return leaderboard;
                }
            }
        }
        Leaderboard { entries: Vec::new() }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_path();
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn add_score(&mut self, name: String, score: u32) {
        self.entries.push(ScoreEntry { name, score });
        self.entries.sort_by(|a, b| b.score.cmp(&a.score));
        self.entries.truncate(Self::MAX_ENTRIES);
    }

    pub fn is_high_score(&self, score: u32) -> bool {
        self.entries.len() < Self::MAX_ENTRIES
            || self.entries.iter().any(|entry| score > entry.score)
    }

    fn get_path() -> PathBuf {
        PathBuf::from(Self::LEADERBOARD_FILE)
    }
}
