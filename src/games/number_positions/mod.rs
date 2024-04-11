use std::path::Path;

use crate::*;

pub fn start(player: &mut Player) {
    let menu = Menu::new("Number Positions", vec!["Easy Mode", "Hard Mode", "Back"]);
    match menu.display() {
        0 => game(player, 4),
        1 => game(player, 8),
        _ => (),
    };
}

fn game(player: &mut Player, length: u8) {
    let target_number: String = {
        let mut number = String::new();
        for _ in 0..length {
            number.push_str(&generate_number(0, 9).to_string());
        }
        number
    };
    let target_chars: Vec<char> = target_number.chars().collect();
    let mut guess_number: String = String::new();
    let mut correct_positions: i32 = 0;
    let mut attempts: i32 = 0;
    loop {
        clear_screen();
        println!("Number Positions");
        println!();
        println!("Attempts: {}", attempts);
        println!("Number Length: {}", length);
        println!("Correct Positions: {}", correct_positions);
        if guess_number.is_empty() {
            println!();
        } else if guess_number == target_number {
            println!();
            println!("Congratulations! You guessed the number!");
            println!("The hidden number was: {}", target_number);
            println!("Your number of attempts: {}", attempts);
            if length > 4 {
                player.add_range_points(70, 100);
            } else {
                player.add_range_points(50, 80);
            }
            player.save(Path::new(PLAYER_FILE_NAME));
            pause_execution();
            break;
        } else {
            println!("Last Guess: {}", guess_number);
            println!();
        }
        guess_number = get_input("Guess >");
        let guess_chars: Vec<char> = guess_number.chars().collect();
        correct_positions = 0;
        for (index, character) in guess_chars.iter().enumerate() {
            if character == &target_chars[index] {
                correct_positions += 1;
            }
        }
        attempts += 1;
    }
}