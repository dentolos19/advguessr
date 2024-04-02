mod games;
mod utils;

use games::*;
use utils::terminal::*;

fn main() {
    let menu = Menu {
        title: "AdvGuessR",
        options: vec!["Games", "Scoreboard (TODO)", "Profile (TODO)", "Exit"],
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