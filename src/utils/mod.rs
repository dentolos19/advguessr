use std::env;

pub mod terminal;
pub mod user;

pub fn go_home() {
    let exe_file_path = env::current_exe().unwrap();
    let exe_dir_path = exe_file_path.parent().unwrap();
    env::set_current_dir(&exe_dir_path).unwrap();
}