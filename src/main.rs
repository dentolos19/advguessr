mod games;
mod utils;

use std::path::Path;

use games::*;
use utils::{terminal::*, user::*, *};

static PLAYER_FILE_NAME: &str = "player.json";

fn main() {
    go_home();

    let player_file_path = Path::new(PLAYER_FILE_NAME);
    let mut player_option = load_player(player_file_path);

    if player_option.is_none() {
        player_option.replace(setup(player_file_path));
    }

    let mut player = player_option.unwrap();

    let menu = Menu::new("AdvGuessR", vec!["Games", "Profile", "Exit"]);
    loop {
        match menu.display() {
            0 => games(&mut player),
            1 => profile(&mut player),
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

fn games(player: &mut Player) {
    let menu = Menu::new(
        "Games",
        vec![
            "Guess The Number",
            "Number Positions",
            "Word Matcher (TODO)",
            "Word Guesser (TODO)",
        ],
    );
    match menu.display() {
        0 => guess_the_number::start(player),
        1 => number_positions::start(player),
        2 => todo!(),
        3 => todo!(),
        _ => (),
    }
}

fn profile(player: &mut Player) {
    let mut menu = Menu::new("Profile", vec!["Back"]);
    menu.description = Some(format!("Name: {}\nScore: {}", player.name, player.score));
    match menu.display() {
        _ => (),
    }
}