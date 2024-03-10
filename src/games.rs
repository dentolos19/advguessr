use crate::utils::*;
use rand::Rng;

pub fn guess_the_number() {
    clear_screen();
    let target_value: u32 = generate_random_number();
    loop {
        let guess_value: u32 = get_number_input("Enter your guess:") as u32;
        if guess_value == target_value {
            println!("You guessed it right!");
            break;
        } else if guess_value < target_value {
            println!("Your guess is too low!");
        } else {
            println!("Your guess is too high!");
        }
    }
    println!();
    println!("Do you want to play again?");
    println!();
    println!("[1] Yes");
    println!("[2] No");
    println!();
    let play_again = get_number_input("Enter your choice:");
    match play_again {
        1 => guess_the_number(),
        _ => return,
    }
}

fn generate_random_number() -> u32 {
    // code for generating random number
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..101);
    random_number
}