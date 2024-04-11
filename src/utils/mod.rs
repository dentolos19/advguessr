use std::env;

use rand::Rng;

pub mod terminal;
pub mod user;

pub fn go_home() {
    let exe_file_path = env::current_exe().unwrap();
    let exe_dir_path = exe_file_path.parent().unwrap();
    env::set_current_dir(&exe_dir_path).unwrap();
}

pub fn generate_number(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}