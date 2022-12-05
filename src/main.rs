extern crate core;

use std::fs;
use crate::days::day5::get_top_items;

mod days;

fn main() {
    // get_max_carrying_calories();
    // get_game_result();
    // get_rucksack_item_priority();
    // get_fully_contain();
    get_top_items()
}

pub fn file_data(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}