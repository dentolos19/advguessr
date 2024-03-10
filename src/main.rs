mod games;
mod utils;

use games::*;
use utils::*;

fn main() {
    loop {
        clear_screen();
        println!("AdvGuessR - The Advanced Guessing Game!");
        println!();
        println!("[1] Login (TODO)");
        println!("[2] Games");
        println!("[3] Scoreboard (TODO)");
        println!("[3] Quit");
        println!();
        let input = get_number_input("> ");
        match input {
            1 => println!("Login not implemented yet!"),
            2 => games(),
            3 => break,
            _ => (),
        }
    }
}

fn games() {
    loop {
        clear_screen();
        println!("Games");
        println!();
        println!("[1] Guess the number");
        println!("[2] Hang Man (TODO)");
        println!("[3] Number Positions (TODO)");
        println!("[4] Color Positions (TODO)");
        println!("[5] Back");
        println!();
        let input = get_number_input("> ");
        match input {
            1 => guess_the_number(),
            2 => break,
            _ => (),
        }
    }
}