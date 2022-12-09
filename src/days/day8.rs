use crate::file_data;

pub fn get_visible_trees() {
    let data = file_data("resources/day8_tree_grid.txt");

    let grid = data.lines().into_iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut visible_trees_count = 0;
    let mut max_scenic_score = 0;

    for i in 1..grid.len()-1 {
       '_inner: for j in 1..grid.get(0).unwrap().len()-1 {
           let tree = grid.get(i).unwrap().get(j).unwrap();


           let to_left_max = grid.get(i).unwrap()[..j].iter().enumerate().max_by_key(|(_idx, &e)| e).unwrap();

           let to_right_max = grid.get(i).unwrap()[j + 1..].iter().enumerate().max_by_key(|(_idx, &e)| e).unwrap();

           let to_up_max = grid.iter()
               .map(|vec| vec.get(j).unwrap())
               .take(i)
               .enumerate()
               .max_by_key(|(_idx, &e)| e).unwrap();

           let to_down_max = grid.iter()
               .map(|vec| vec.get(j).unwrap())
               .skip(i + 1)
               .enumerate()
               .max_by_key(|(_idx, &e)| e).unwrap();

           let mut scenic_score = 1;
           if tree > to_left_max.1 {
               scenic_score *= j;
           } else {
               scenic_score *= j - to_left_max.0;
           }

           if tree > to_right_max.1 {
               scenic_score *= grid.get(0).unwrap().len() - 1 - j;
           } else {
               scenic_score *= to_right_max.0 - j;
           }

           if tree > to_up_max.1 {
               scenic_score *= i;
           } else {
               scenic_score *= i - to_up_max.0;
           }

           if tree > to_down_max.1 {
               scenic_score *= grid.len() - 1 - i;
           } else {
               scenic_score *= to_down_max.0 - i;
           }

           if scenic_score > max_scenic_score {
               max_scenic_score = scenic_score;
           }

           println!()
       }
    }

    println!("Visible trees {}", visible_trees_count + ((grid.len()-1) * 4));
    println!("max scenic score {}", max_scenic_score );

}
