use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

impl Player {
    pub fn add_points(&mut self, score: i32) {
        self.score += score;
    }

    pub fn add_range_points(&mut self, min: i32, max: i32) {
        self.add_points(generate_number(min, max));
    }

    pub fn save(&self, path: &Path) {
        let json = serde_json::to_string(&self).unwrap();
        fs::write(path, json).unwrap();
    }
}

pub fn load_player(path: &Path) -> Option<Player> {
    if !path.exists() {
        // let player = Player {
        //     name: String::new(),
        //     score: 0,
        // };
        // return player;
        return None;
    }
    let contents = fs::read_to_string(path).unwrap();
    let player: Player = serde_json::from_str(&contents).unwrap();
    Some(player)
}