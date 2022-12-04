use crate::file_data;

pub fn get_rucksack_item_priority() {
    let data = file_data("resources/day3_rucsacks.txt");

    let mut item_char_vec: Vec<char> = vec![];
    let mut groups: Vec<Vec<&str>> = vec![vec![]];

    let mut group_index = 0;
    let mut triple_index = 0;

    for  line in data.lines() {
        let half_len = line.len() / 2;
        let left = &line[0..half_len];
        let right = &line[half_len..line.len()];

        '_outer: for left_char in left.chars() {
             for right_char in right.chars() {
                if left_char == right_char {
                    item_char_vec.push(left_char);
                    break '_outer;
                }
            }
        }

        match triple_index {
            3 => {
                group_index += 1;
                triple_index = 0;
                groups.push(vec![]);
            },
            _ => {}
        }

        let group = groups.get_mut(group_index).unwrap();
        group.push(line);

        triple_index += 1;

    }

    let mut priority_sum = 0;

    for item in &item_char_vec {
        if item.is_ascii_lowercase() {
            priority_sum += ((*item as u8) - b'a') as usize + 1
        } else if item.is_ascii_uppercase() {
            priority_sum += ((*item as u8) - b'A') as usize + 27
        } else {
            priority_sum += 0;
        }
    }

    let mut group_sum = 0;
    println!("{:?}", &groups.get(0).unwrap());

    for group in groups.iter_mut() {
        group.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
    }
    println!("================================================");
    println!("{:?}", &groups.get(0).unwrap());

    '_first: for group in &groups {
        for c1 in group.get(0).unwrap().chars() {
            for c2 in group.get(1).unwrap().chars() {
                for c3 in group.get(2).unwrap().chars() {
                    if c1 == c2 && c2 == c3 && c1 == c3 {
                        print!("group {:?} common = {}", group, c1);
                        if c1.is_ascii_lowercase() {
                            group_sum += ((c1 as u8) - b'a') as usize + 1;
                            print!(" priority {}", ((c1 as u8) - b'a') as usize + 1)
                        } else if c1.is_ascii_uppercase() {
                            group_sum += ((c1 as u8) - b'A') as usize + 27;
                            print!(" priority {}", ((c1 as u8) - b'A') as usize + 27)
                        } else {
                            group_sum += 0;
                        }
                        println!();
                        continue '_first;
                    }
                }
            }
        }
    }


    // println!("{:?}", &item_char_vec);

    println!("Priority sum = {:?}", priority_sum);
    println!("Group sum = {:?}", group_sum);
}