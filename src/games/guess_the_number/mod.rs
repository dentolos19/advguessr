use crate::utils::randomization::*;
use crate::utils::terminal::*;

pub fn start() {
    let menu = MenuData {
        title: "Guess The Number".to_string(),
        options: vec![
            "Easy Mode (0 to 100)".to_string(),
            "Hard Mode (0 to 1000)".to_string(),
            "Exit".to_string(),
        ],
    };
    match menu.display() {
        1 => game(0, 100),
        2 => game(0, 1000),
        _ => (),
    }
}

fn game(min: usize, max: usize) {
    let target_number = generate_random_number(min, max) as i32;
    let mut guess_number = -1;
    let mut attempts = 0;
    loop {
        clear_screen();
        println!("Guess The Number");
        println!();
        println!("Range: {} to {}", min, max);
        println!("Attempts: {}", attempts);
        println!();
        if guess_number < 0 {
            // Do nothing
        } else if guess_number == target_number {
            println!("Congratulations! You guessed the number!");
            println!("Attempts: {}", attempts);
            println!();
            pause_terminal();
            break;
        } else if guess_number < target_number {
            println!("{} The number is higher.", guess_number);
            println!();
        } else {
            println!("{} The number is lower.", guess_number);
            println!();
        }

        guess_number = get_number_input(">");
        attempts += 1;
    }
}