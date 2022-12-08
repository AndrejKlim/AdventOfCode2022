use std::fs;
use crate::days::day8::get_visible_trees;

mod days;

fn main() {
    // get_max_carrying_calories();
    // get_game_result();
    // get_rucksack_item_priority();
    // get_fully_contain();
    // get_top_items()
    // get_chars_processed()
    // TODO  get_dirs_size()
    get_visible_trees();
}

pub fn file_data(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}