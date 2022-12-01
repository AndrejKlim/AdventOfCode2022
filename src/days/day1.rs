use crate::file_data;

pub fn get_max_carrying_calories() {
    let data = file_data("resources/day1_calories.txt");
    // println!("{}", &data);

    let mut backpacs: Vec<Vec<i32>> = vec![vec![]];

    let mut counter = 0;
    for str in data.lines() {
        if str.is_empty() {
            counter += 1;
        }

        if let Some(vec) = backpacs.get_mut(counter) {
            vec.push(str.parse::<i32>().unwrap());
        } else {
            backpacs.push(vec![]);
        }
    }
    // println!("{:?}", &backpacs);

    backpacs.sort_by(|a, b| a.iter().sum::<i32>().cmp(&b.iter().sum::<i32>()).reverse() );

    let sum_max_three = backpacs.get(0).unwrap().iter().sum::<i32>()
        + backpacs.get(1).unwrap().iter().sum::<i32>()
        + backpacs.get(2).unwrap().iter().sum::<i32>();

    print!("Max calories backpack sum: {:?}", sum_max_three);
}