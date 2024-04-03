use std::{fs, path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
    pub score: i32,
}

pub fn load_users(path: &path::Path) -> Vec<User> {
    if !path.exists() {
        std::fs::write(path, "[]").unwrap();
    }
    let contents = fs::read_to_string(path).unwrap();
    let users: Vec<User> = serde_json::from_str(&contents).unwrap();
    users
}