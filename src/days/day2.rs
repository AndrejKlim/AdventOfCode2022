use crate::file_data;

pub fn get_game_result() {
    let data = file_data("resources/day2_rock_paper_scissors.txt");
    // println!("{}", &data);
    // A for Rock, B for Paper, and C for Scissors.
    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
    // X for Rock, Y for Paper, and Z for Scissors.
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // 0 if you lost, 3 if the round was a draw, and 6 if you won).

    let mut game_result: i32 = 0;
    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
    let mut fake_game_result: i32 = 0;

    for pair in data.lines() {
        let mut splitten_line = pair.splitn(2, " ");
        let tuple = (splitten_line.next().unwrap(), splitten_line.next().unwrap());

        match tuple {
            ("A", "X") => {
                game_result += 1 + 3;
                fake_game_result += 0 + 3;
            }
            ("B", "X") => {
                game_result += 1 + 0;
                fake_game_result += 0 + 1;
            }
            ("C", "X") => {
                game_result += 1 + 6;
                fake_game_result += 0 + 2;
            }
            ("A", "Y") => {
                game_result += 2 + 6;
                fake_game_result += 3 + 1;
            }
            ("B", "Y") => {
                game_result += 2 + 3;
                fake_game_result += 3 + 2;
            }
            ("C", "Y") => {
                game_result += 2 + 0;
                fake_game_result += 3 + 3;
            }
            ("A", "Z") => {
                game_result += 3 + 0;
                fake_game_result += 6 + 2;
            }
            ("B", "Z") => {
                game_result += 3 + 6;
                fake_game_result += 6 + 3;
            }
            ("C", "Z") => {
                game_result += 3 + 3;
                fake_game_result += 6 + 1;
            }
            (_, _) => {}
        }
    }

    println!("Game result = {:?}", game_result);
    println!("Fake game result = {:?}", fake_game_result);
}