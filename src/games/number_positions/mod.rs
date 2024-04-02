use rand::Rng;

use crate::utils::terminal::*;

pub fn start() {
    let menu = Menu {
        title: "Number Positions",
        options: vec!["Easy Mode", "Hard Mode", "Back"],
    };
    match menu.display() {
        0 => game(4),
        1 => game(8),
        _ => (),
    };
}

fn game(length: u8) {
    let target_number: String = {
        let mut number = String::new();
        for _ in 0..length {
            number.push_str(&rand::thread_rng().gen_range(0..10).to_string());
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