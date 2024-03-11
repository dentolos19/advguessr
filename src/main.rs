#![allow(dead_code)]

mod games;
mod utils;

use games::*;
use utils::terminal::*;

fn main() {
    let menu = Menu {
        title: String::from("AdvGuessR"),
        options: vec![
            String::from("Games"),
            String::from("Scoreboard"),
            String::from("Profile"),
            String::from("Exit"),
        ],
    };
    loop {
        match menu.display() {
            0 => games(),
            1 => todo!(),
            2 => todo!(),
            3 => break,
            _ => (),
        }
    }
}

fn games() {
    let menu = Menu {
        title: String::from("Games"),
        options: vec![
            String::from("Guess The Number"),
            String::from("Number Positions"),
            String::from("Back"),
        ],
    };
    match menu.display() {
        0 => guess_the_number::start(),
        1 => number_positions::start(),
        _ => (),
    }
}