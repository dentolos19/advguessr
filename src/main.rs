mod games;
mod utils;

use std::path::Path;

use games::*;
use utils::{terminal::*, user::*, *};

fn main() {
    go_home();

    let player_file_path = Path::new("player.json");
    let mut player = load_player(player_file_path);

    if player.is_none() {
        player.replace(setup(player_file_path));
    }

    let menu = Menu::new("AdvGuessR", vec!["Games", "Profile", "Exit"]);
    loop {
        match menu.display() {
            0 => games(),
            1 => profile(player.as_ref().unwrap()),
            2 => break,
            _ => (),
        }
    }
}

fn setup(path: &Path) -> Player {
    let player = Player {
        name: get_input("Enter your name: "),
        score: 0,
    };
    player.save(path);
    player
}

fn games() {
    let menu = Menu::new(
        "Games",
        vec![
            "Guess The Number",
            "Number Positions",
            "Word Matcher",
            "Word Guesser",
        ],
    );
    match menu.display() {
        0 => guess_the_number::start(),
        1 => number_positions::start(),
        2 => todo!(),
        3 => todo!(),
        _ => (),
    }
}

fn profile(profile: &Player) {
    let mut menu = Menu::new("Profile", vec!["Back"]);
    menu.description = Some(format!("Name: {}\nScore: {}", profile.name, profile.score));
    match menu.display() {
        _ => (),
    }
}