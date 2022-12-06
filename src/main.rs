use std::fs;
use crate::days::day6::get_chars_processed;

mod days;

fn main() {
    // get_max_carrying_calories();
    // get_game_result();
    // get_rucksack_item_priority();
    // get_fully_contain();
    // get_top_items()
    get_chars_processed()
}

pub fn file_data(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}