use crate::file_data;

pub fn get_visible_trees() {
    let data = file_data("resources/day8_tree_grid.txt");

    let grid = data.lines().into_iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut visible_trees_count = 0;

    for i in 1..grid.len()-1 {
       '_inner: for j in 1..grid.get(0).unwrap().len()-1 {
           let tree = grid.get(i).unwrap().get(j).unwrap();

           let to_left_max = grid.get(i).unwrap()[..j].iter().max().unwrap();
           if tree > to_left_max {
               visible_trees_count += 1;
               continue '_inner;
           }
           let to_right_max = grid.get(i).unwrap()[j + 1..].iter().max().unwrap();
           if tree > to_right_max {
               visible_trees_count += 1;
               continue '_inner;
           }
           let to_up_max = grid.iter()
               .map(|vec| vec.get(j).unwrap())
               .take(i)
               .max().unwrap();
           if tree > to_up_max {
               visible_trees_count += 1;
               continue '_inner;
           }
           let to_down_max = grid.iter()
               .map(|vec| vec.get(j).unwrap())
               .skip(i + 1)
               .max().unwrap();
           if tree > to_down_max {
               visible_trees_count += 1;
           }
       }
    }

    println!("Visible trees {}", visible_trees_count + ((grid.len()-1) * 4));

}
