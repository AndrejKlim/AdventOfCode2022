use std::fs;
use crate::days::day3::get_rucksack_item_priority;

mod days;

fn main() {
    // get_max_carrying_calories();
    // get_game_result();
    get_rucksack_item_priority();
    println!("{}", 'a'.is_ascii_lowercase());
}

pub fn file_data(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}