use crate::file_data;

pub fn get_chars_processed() {
    let data = file_data("resources/day6_signals.txt");

    let arr = data.chars().collect::<Vec<char>>();
    let mut char_processed = 0;

    for index in 0..arr.len() - 4 {
        let mut same_char_found = false;
        '_inner: for (char_index, char) in arr.as_slice()[index..index+4].into_iter().enumerate() {
            for char_inner in arr.as_slice()[index+char_index+1..index+4].into_iter() {
                if char == char_inner {
                    same_char_found = true;
                    break '_inner;
                }
            }
        }
        if !same_char_found {
            char_processed = index+4;
            break;
        }
    }

    print!("Chars processed {}", char_processed);
}