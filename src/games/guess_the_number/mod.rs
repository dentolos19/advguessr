use crate::utils::terminal::*;
use rand::*;

pub fn start() {
    let menu = Menu {
        title: "Guess The Number",
        options: vec!["Easy Mode", "Hard Mode", "Back"],
    };
    match menu.display() {
        0 => game(0, 100),
        1 => game(0, 2000),
        _ => (),
    }
}

fn game(min: i32, max: i32) {
    let target_number: i32 = rand::thread_rng().gen_range(min..(max + 1));
    let mut guess_number: i32 = -1;
    let mut attempts: i32 = 0;
    loop {
        clear_screen();
        println!("Guess The Number");
        println!();
        println!("Range: {} to {}", min, max);
        println!("Attempts: {}", attempts);
        if guess_number < 0 {
            println!();
        } else if guess_number == target_number {
            println!();
            println!("Congratulations! You guessed the number!");
            println!("The hidden number was: {}", target_number);
            println!("Your number of attempts: {}", attempts);
            pause_execution();
            break;
        } else if guess_number < target_number {
            println!();
            println!("The number is greater than {}!", guess_number);
            println!();
        } else {
            println!();
            println!("The number is less than {}!", guess_number);
            println!();
        }
        guess_number = get_number_input("Guess >");
        attempts += 1;
    }
}