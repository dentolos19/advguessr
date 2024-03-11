use core::time;
use crossterm::*;
use std::io::*;

pub struct Menu {
    pub title: String,
    pub options: Vec<String>,
}

impl Menu {
    pub fn display(&self) -> isize {
        let mut current_selection: usize = 0;
        loop {
            clear_screen();
            println!("{}", self.title);
            println!();
            for (index, option) in self.options.iter().enumerate() {
                if index == current_selection {
                    println!("> {}", option);
                } else {
                    println!("  {}", option);
                }
            }
            match detect_key() {
                Ok(event::KeyCode::Up) => {
                    if current_selection > 0 {
                        current_selection -= 1;
                    }
                }
                Ok(event::KeyCode::Down) => {
                    if current_selection < self.options.len() - 1 {
                        current_selection += 1;
                    } else {
                        current_selection = 0;
                    }
                }
                Ok(event::KeyCode::Enter) => {
                    return current_selection as isize;
                }
                Ok(event::KeyCode::Esc) => {
                    return -1;
                }
                Ok(_) => (),
                Err(_) => {
                    return -1;
                }
            }
        }
    }
}

fn detect_key() -> Result<event::KeyCode> {
    loop {
        if event::poll(time::Duration::from_millis(500))? {
            match event::read() {
                Ok(event::Event::Key(event)) => {
                    if event.kind == event::KeyEventKind::Press {
                        return Ok(event.code);
                    }
                }
                Ok(_) => (),
                Err(error) => return Err(error),
            }
        }
    }
}

pub fn clear_screen() {
    // idk what the fuck is going on, but it works
    if cfg!(windows) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    } else {
        print!("{esc}c", esc = 27 as char);
    }
}

pub fn pause_execution() {
    detect_key().unwrap();
}

pub fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{} ", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_number_input(prompt: &str) -> i32 {
    let input = get_input(prompt);
    match input.parse() {
        Ok(number) => return number,
        Err(_) => return -1,
    }
}