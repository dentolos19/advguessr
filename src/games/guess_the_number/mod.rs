use std::path::Path;

use crate::*;

pub fn start(player: &mut Player) {
    let menu = Menu::new("Guess The Number", vec!["Easy Mode", "Hard Mode", "Back"]);
    match menu.display() {
        0 => game(player, 0, 100),
        1 => game(player, 0, 2000),
        _ => (),
    }
}

fn game(player: &mut Player, min: i32, max: i32) {
    let target_number: i32 = generate_number(min, max);
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
            if max > 500 {
                player.add_range_points(70, 100);
            } else {
                player.add_range_points(50, 80);
            }
            player.save(Path::new(PLAYER_FILE_NAME));
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