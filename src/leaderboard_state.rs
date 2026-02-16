use crate::persistence::Leaderboard;

pub struct LeaderboardState {
    pub leaderboard: Leaderboard,
}

impl LeaderboardState {
    pub fn new() -> Self {
        LeaderboardState {
            leaderboard: Leaderboard::load(),
        }
    }

    pub fn refresh(&mut self) {
        self.leaderboard = Leaderboard::load();
    }
}
