mod games;
mod utils;

use std::path::Path;

use games::*;
use utils::{*, terminal::*, user::*};

fn main() {
    // Sets current working directory to the executable's directory
    go_home();

    let users: Vec<User> = load_users(Path::new("users.json"));

    let menu = Menu {
        title: "AdvGuessR",
        options: vec!["Games", "Scoreboard (TODO)", "Profile (TODO)", "Exit"]
    };
    loop {
        match menu.display() {
            0 => games(),
            1 => scoreboard(),
            2 => todo!(),
            3 => break,
            _ => (),
        }
    }
}

fn games() {
    let menu = Menu {
        title: "Games",
        options: vec![
            "Guess The Number",
            "Number Positions",
            "Word Matcher",
            "Word Guesser",
        ],
    };
    match menu.display() {
        0 => guess_the_number::start(),
        1 => number_positions::start(),
        2 => todo!(),
        3 => todo!(),
        _ => (),
    }
}

fn scoreboard() {
    todo!();
}