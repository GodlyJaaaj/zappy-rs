use iced::Color;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct GameState {
    pub map_width: Option<u32>,
    pub map_height: Option<u32>,

    pub teams: Vec<(String, Color)>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            map_width: None,
            map_height: None,
            teams: vec![],
        }
    }
}

impl GameState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn update_map_size(&mut self, width: u32, height: u32) {
        self.map_width = Some(width);
        self.map_height = Some(height);
    }

    pub fn add_team(&mut self, team: String) {
        let mut rng = rand::rng();
        let random_color = Color::from_rgb(
            rng.random_range(0.2..=0.8),
            rng.random_range(0.2..=0.8),
            rng.random_range(0.2..=0.8),
        );

        self.teams.push((team, random_color));
    }
}
