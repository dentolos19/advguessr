#![allow(dead_code)]
#![allow(unreachable_code)]

mod games;
mod utils;

use games::*;
use utils::terminal::*;

fn main() {
    loop {
        let menu = MenuData {
            title: "Main Menu".to_string(),
            options: vec!["Login".to_string(), "Games".to_string(), "Quit".to_string()],
        };
        match menu.display() {
            1 => (), // TODO: show login menu
            2 => games(),
            _ => break,
        }
    }
}

fn games() {
    let menu = MenuData {
        title: "Games".to_string(),
        options: vec![
            "Guess The Number".to_string(),
            "Number Positions".to_string(),
            "Back".to_string()
        ]
    };
    match menu.display() {
        1 => guess_the_number::start(),
        2 => number_positions::start(),
        _ => (),
    }
}