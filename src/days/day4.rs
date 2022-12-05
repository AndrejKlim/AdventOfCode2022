use std::collections::HashSet;
use crate::file_data;

pub fn get_fully_contain() {
    let data = file_data("resources/day4_clearing.txt");


    let mut fully_contain = 0;
    let mut overlaps = 0;

    for  line in data.lines() {
        let mut pair = line.split(",");
        let mut left = pair.next().unwrap().split("-");
        let first_pair: (i32, i32) = (left.next().unwrap().parse().unwrap(), left.next().unwrap().parse().unwrap());

        let mut right = pair.next().unwrap().split("-");
        let sec_pair: (i32, i32) = (right.next().unwrap().parse().unwrap(), right.next().unwrap().parse().unwrap());

        if first_pair.0 >= sec_pair.0 && first_pair.1 <= sec_pair.1 ||
            sec_pair.0 >= first_pair.0 && sec_pair.1 <= first_pair.1 {
            fully_contain += 1;
        }

        let set_first: HashSet<i32> = (first_pair.0 .. first_pair.1 + 1).collect();
        let set_second: HashSet<i32> = (sec_pair.0 .. sec_pair.1 + 1).collect();

        let intersect = set_first.intersection(&set_second)
            .map(|i| *i)
            .collect::<Vec<i32>>();

        println!("{:?}", intersect);

        if !intersect.is_empty() { overlaps += 1 }

    }

    println!("Fully contain in pairs = {:?}", fully_contain);
    println!("Overlaps = {:?}", overlaps);
}