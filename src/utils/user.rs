use std::{fs, path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

impl Player {
    pub fn save(&self, path: &path::Path) {
        let json = serde_json::to_string(&self).unwrap();
        fs::write(path, json).unwrap();
    }
}

pub fn load_player(path: &path::Path) -> Option<Player> {
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