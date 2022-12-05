use crate::file_data;

pub fn get_top_items() {
    let data = file_data("resources/day5_moves.txt");

    /*  [Q]         [N]             [N]
        [H]     [B] [D]             [S] [M]
        [C]     [Q] [J]         [V] [Q] [D]
        [T]     [S] [Z] [F]     [J] [J] [W]
        [N] [G] [T] [S] [V]     [B] [C] [C]
        [S] [B] [R] [W] [D] [J] [Q] [R] [Q]
        [V] [D] [W] [G] [P] [W] [N] [T] [S]
        [B] [W] [F] [L] [M] [F] [L] [G] [J]
         1   2   3   4   5   6   7   8   9 */

    let mut crates = vec![
        vec!["B", "V", "S", "N", "T", "C", "H", "Q"],
        vec!["W", "D", "B", "G"],
        vec!["F", "W", "R", "T", "S", "Q", "B"],
        vec!["L", "G", "W", "S", "Z", "J", "D", "N"],
        vec!["M", "P", "D", "V", "F"],
        vec!["F", "W", "J"],
        vec!["L", "N", "Q", "B", "J", "V"],
        vec!["G", "T", "R", "C", "J", "Q", "S", "N"],
        vec!["J", "S", "Q", "C", "W", "D", "M"]];

    for (i, command) in data.lines().enumerate() {
        let split = command.split(" ");
        let mut command_index = 0;
        let mut command: [usize; 3]  = [0; 3];
        for part in split.into_iter() {
            let parse_result = part.parse::<usize>();
            if let Ok(num) = parse_result {
                command[command_index] = num;
                command_index += 1;
            }
        }


        let from = crates.get_mut(command[1] - 1).unwrap();

        let temp_vec = from.as_slice()[from.len()-command[0]..].to_vec();

        println!("{:?}", temp_vec);

        for _i in 0..command[0] {
            from.pop();
        }

        for item in temp_vec {
            let to = crates.get_mut(command[2] - 1).unwrap();

            to.push(item);
        }

        println!("{:?}", command);
        println!("Iteration {}" , i);
        for c in &crates {
            println!("{:?}", c)
        }
        println!();
    }

    let mut result = "".to_string();
    for c in &crates {
        let last = c.last().unwrap();
        result.push_str(last);
    }

    print!("Result {}", result);
}

// println!("Fully contain in pairs = {:?}", fully_contain);
// println!("Overlaps = {:?}", overlaps);
