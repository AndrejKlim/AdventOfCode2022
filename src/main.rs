use std::fs;
use crate::days::day2::get_game_result;

mod days;

fn main() {
    // get_max_carrying_calories();
    get_game_result();
}

pub fn file_data(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}