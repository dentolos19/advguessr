use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use std::{io::Write, process};

pub struct MenuData {
    pub title: String,
    pub options: Vec<String>,
}

impl MenuData {
    pub fn display(&self) -> i32 {
        let mut current_selection: i32 = 1;
        loop {
            clear_screen();
            println!("{}", self.title);
            println!();
            for (index, option) in self.options.iter().enumerate() {
                if index as i32 + 1 == current_selection {
                    print_flush("> ");
                }
                println!("{}", option);
            }
            // println!();
            // let input: i32 = get_number_input(&self.prompt);
            // if input > 0 && input <= self.options.len() as i32 {
            //     return input;
            // }
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    if current_selection > 1 {
                        current_selection -= 1;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    if current_selection < self.options.len() as i32 {
                        current_selection += 1;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    return current_selection;
                }
                _ => (),
            }
        }
    }
}

fn clear_screen() {
    if cfg!(windows) {
        process::Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .unwrap();
    } else {
        process::Command::new("clear").status().unwrap();
    }
}

fn print_flush(text: &str) {
    print!("{}", text);
    std::io::stdout().flush().unwrap(); // Refer to https://stackoverflow.com/a/67185950
}

fn get_input(prompt: &str) -> String {
    print_flush(format!("{} ", prompt).as_str());
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}

fn get_number_input(prompt: &str) -> i32 {
    let input: String = get_input(prompt);
    match input.parse::<i32>() {
        Ok(number) => number,
        Err(_) => -1,
    }
}