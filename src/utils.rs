use std::process;
use std::io::Write;

pub fn clear_screen() {
    if cfg!(windows) {
        let _ = process::Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = process::Command::new("clear").status();
    }
}

pub fn get_number_input(prompt: &str) -> i32 {
    print!("{} ", prompt);
    std::io::stdout().flush().unwrap(); // Refer to https://stackoverflow.com/a/67185950
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => -1,
    }
}