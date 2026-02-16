#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    Start,
    Leaderboard,
    Exit,
}

pub struct MenuState {
    pub selected_item: MenuItem,
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            selected_item: MenuItem::Start,
        }
    }

    pub fn move_up(&mut self) {
        self.selected_item = match self.selected_item {
            MenuItem::Start => MenuItem::Exit,
            MenuItem::Leaderboard => MenuItem::Start,
            MenuItem::Exit => MenuItem::Leaderboard,
        };
    }

    pub fn move_down(&mut self) {
        self.selected_item = match self.selected_item {
            MenuItem::Start => MenuItem::Leaderboard,
            MenuItem::Leaderboard => MenuItem::Exit,
            MenuItem::Exit => MenuItem::Start,
        };
    }

    pub fn get_items() -> Vec<MenuItem> {
        vec![MenuItem::Start, MenuItem::Leaderboard, MenuItem::Exit]
    }
}
