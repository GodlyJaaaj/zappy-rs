#[derive(Debug, Clone)]
pub struct GameState {
    pub map_width: Option<u32>,
    pub map_height: Option<u32>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            map_width: None,
            map_height: None,
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
}
