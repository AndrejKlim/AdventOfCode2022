use std::fs;
use crate::days::day1::get_max_carrying_calories;

mod days;

fn main() {
    get_max_carrying_calories();
}

pub fn file_data(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}