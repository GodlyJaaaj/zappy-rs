use iced::Color;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u64,                  // Identifiant unique du joueur
    pub team_index: usize,        // Index de l'équipe dans `GameState::teams`
    pub position: (u64, u64),     // Position
    pub orientation: Orientation, // Orientation
    pub level: u8,                // Level
}

impl TryFrom<u8> for Orientation {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Orientation::North),
            2 => Ok(Orientation::East),
            3 => Ok(Orientation::South),
            4 => Ok(Orientation::West),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    map_width: Option<u32>,
    map_height: Option<u32>,

    teams: Vec<(String, Color)>,
    players: HashMap<u64, Player>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            map_width: None,
            map_height: None,
            teams: vec![],
            players: HashMap::new(),
        }
    }
}

impl GameState {
    pub fn get_team_for_player(&self, player: &Player) -> &(String, Color) {
        &self.teams[player.team_index]
    }

    pub fn add_player(
        &mut self,
        id: u64,
        team_index: usize,
        position: (u64, u64),
        orientation: Orientation,
        level: u8,
    ) {
        if team_index >= self.teams.len() {
            panic!("Invalid team index: {}", team_index);
        }
        let player = Player {
            id,
            team_index,
            position,
            orientation,
            level,
        };

        self.players.insert(id, player);
    }

    pub fn update_map_size(&mut self, width: u32, height: u32) {
        self.map_width = Some(width);
        self.map_height = Some(height);
    }

    pub fn add_team(&mut self, team: String) {
        let mut rng = rand::rng();
        let mut random_color = Color::from_rgb(
            rng.random_range(0.2..=0.8),
            rng.random_range(0.2..=0.8),
            rng.random_range(0.2..=0.8),
        );
        
        random_color.a = 0.5;

        self.teams.push((team, random_color));
    }

    pub fn teams(&self) -> &Vec<(String, Color)> {
        &self.teams
    }

    pub fn width(&self) -> Option<u32> {
        self.map_width
    }

    pub fn height(&self) -> Option<u32> {
        self.map_height
    }

    pub fn players(&self) -> &HashMap<u64, Player> {
        &self.players
    }

    pub fn players_mut(&mut self) -> &mut HashMap<u64, Player> {
        &mut self.players
    }

    pub fn remove_player(&mut self, id: u64) {
        self.players.remove(&id);
    }
}
