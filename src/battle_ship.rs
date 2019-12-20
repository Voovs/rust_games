use std::io;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn main() {
  let mut p1_ships = [[0; 6]; 6];
  let mut p2_ships = [[0; 6]; 6];
  let mut p1_guesses = [[" "; 6]; 6];
  let mut p2_guesses = [[" "; 6]; 6];

  setup::ready_ships_and_intro(&mut p1_ships, &mut p2_ships);

  let mut is_p1_turn = true;

  while !is_win() {
      if is_p1_turn {
          run_game::player_move(&mut p1_guesses, &mut p2_ships);

      }
      else {
          run_game::player_move(&mut p2_guesses, &mut p1_ships);

      }
      is_p1_turn = !is_p1_turn;
  }
}

mod reuse {
    pub fn print_ship_map(grid: &[[usize; 6]; 6]) {
        let mut g = [[" "; 6]; 6];

        for column in 0..6 {
            for row in 0..6 {
                g[column][row] = match grid[column][row] {
                    0 => " ",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    _ => panic!("Match catch fired @ print_ship_map()"),
                }
            }
        }

        println!("");
        println!("     A   B   C   D   E   F");
        println!("  1  {} | {} | {} | {} | {} | {}", g[0][0], g[1][0], g[2][0], g[3][0], g[4][0], g[5][0]);
        println!("  2  {} | {} | {} | {} | {} | {}", g[0][1], g[1][1], g[2][1], g[3][1], g[4][1], g[5][1]);
        println!("  3  {} | {} | {} | {} | {} | {}", g[0][2], g[1][2], g[2][2], g[3][2], g[4][2], g[5][2]);
        println!("  4  {} | {} | {} | {} | {} | {}", g[0][3], g[1][3], g[2][3], g[3][3], g[4][3], g[5][3]);
        println!("  5  {} | {} | {} | {} | {} | {}", g[0][4], g[1][4], g[2][4], g[3][4], g[4][4], g[5][4]);
        println!("  6  {} | {} | {} | {} | {} | {}", g[0][5], g[1][5], g[2][5], g[3][5], g[4][5], g[5][5]);
        println!("");
    }
    pub fn print_guess_map(grid: &[[str; 6]; 6]) {
        let g = grid;

        println!("");
        println!("     A   B   C   D   E   F");
        println!("  1  {} | {} | {} | {} | {} | {}", g[0][0], g[1][0], g[2][0], g[3][0], g[4][0], g[5][0]);
        println!("  2  {} | {} | {} | {} | {} | {}", g[0][1], g[1][1], g[2][1], g[3][1], g[4][1], g[5][1]);
        println!("  3  {} | {} | {} | {} | {} | {}", g[0][2], g[1][2], g[2][2], g[3][2], g[4][2], g[5][2]);
        println!("  4  {} | {} | {} | {} | {} | {}", g[0][3], g[1][3], g[2][3], g[3][3], g[4][3], g[5][3]);
        println!("  5  {} | {} | {} | {} | {} | {}", g[0][4], g[1][4], g[2][4], g[3][4], g[4][4], g[5][4]);
        println!("  6  {} | {} | {} | {} | {} | {}", g[0][5], g[1][5], g[2][5], g[3][5], g[4][5], g[5][5]);
        println!("");
    }
    // pub fn is_win(grid: &[[usize; 6]; 6]) -> bool {
    //     for column in grid {
    //         for row in column {
    //             if grid[column][row] == 0 { continue; }
    //             else { return false; }
    //         }
    //     }
    //
    //     true
    // }
    pub fn move_to_point_success(p_move: &String) -> Option<(usize, usize)> {
        // Prevents slice panic on insufficient user input string length
        if p_move.len() < 11 { return None; }

        let column = &p_move[0..1];
        let row = &p_move[1..2];

        let row_num = match row {
            "1" => row.parse::<usize>().unwrap(),
            "2" => row.parse::<usize>().unwrap(),
            "3" => row.parse::<usize>().unwrap(),
            "4" => row.parse::<usize>().unwrap(),
            "5" => row.parse::<usize>().unwrap(),
            "6" => row.parse::<usize>().unwrap(),
            _ => { return None; },
        };

        // let row_num = row.parse::<usize>().unwrap();
        if !(0 < row_num && row_num < 7) { return None; }

        let column_num: usize = match column {
            "a" => 1,
            "b" => 2,
            "c" => 3,
            "d" => 4,
            "e" => 5,
            "f" => 6,
            _  => { return None; },
        };

        Some((column_num, row_num))
    }
    pub fn slice_off_last_char(string: &String) -> String {
          let string = &string[..(string.len() - 1)].to_string();

          string.to_string();

          // let string = *string;

          string.to_owned()
        }
}

mod setup {
    use std::io;

    pub fn ready_ships_and_intro(
        mut p1_ships: &mut [[usize; 6]; 6],
        mut p2_ships: &mut [[usize; 6]; 6])
        -> () {
            {
                // print!("{}[2J", 27 as char); // Clears cmd screen
                println!("It's player 1's turn. Get ready to place your ships");
                println!("");

                user_place_ships(&mut p1_ships);

                super::reuse::print_ship_map(&p1_ships);

                println!("Player 1's ships have been placed");
            }
            {
                // print!("{}[2J", 27 as char); // Clears cmd screen
                println!("It's player 2's turn. Get ready to place your ships");
                println!("");

                user_place_ships(&mut p2_ships);

                super::reuse::print_ship_map(&p2_ships);

                println!("Player 2's ships have been placed");
            }
        println!("All ships are now placed. \"The game is on\" — Sherlock");
    }

    #[allow(unused_assignments)]
    fn user_place_ships(mut p_ships: &mut [[usize; 6]; 6]) {
        let mut ship_2_place = String::new();
        let mut ship_3_place = String::new();
        let mut ship_4_place = String::new();

        super::reuse::print_ship_map(&p_ships);

        loop {
            ship_2_place = String::new();

            println!("Place your 2 tile ship (X X)");

            io::stdin().read_line(&mut ship_2_place)
            .expect("Didn't read anything");

            ship_2_place = super::reuse::slice_off_last_char(&ship_2_place);

            if place_ships_success(&mut p_ships, &ship_2_place, 2) { break; }
        }
        super::reuse::print_ship_map(&p_ships);

        loop {
            ship_3_place = String::new();

            println!("Place your 3 tile ship (X X X)");

            io::stdin().read_line(&mut ship_3_place)
            .expect("Didn't read anything");

            ship_3_place = super::reuse::slice_off_last_char(&ship_3_place);

            if place_ships_success(&mut p_ships, &ship_3_place, 3) { break; }
        }
        super::reuse::print_ship_map(&p_ships);

        loop {
            ship_4_place = String::new();

            println!("Place your 4 tile ship (X X X X)");

            io::stdin().read_line(&mut ship_4_place)
            .expect("Didn't read anything");

            ship_4_place = super::reuse::slice_off_last_char(&ship_4_place);

            if place_ships_success(&mut p_ships, &ship_4_place, 4) { break; }
        }
        super::reuse::print_ship_map(&p_ships);


        println!("Well done. All your ships are now set");
    }

    fn is_valid_move(
        p_move: &String,
        ship_length: usize,
        grid: &[[usize; 6]; 6])
        -> bool {
        // Prevents slice panic on insufficient user input string length
        if p_move.len() < 11 {
            println!("That's not proper syntax for a move. Ex:       a3 vertical");
            println!("");
            return false;
        }

        let direction = &p_move[3..];

        let mut column_num: usize;
        let mut row_num: usize;

        if let Some((c, r)) = super::reuse::move_to_point_success(p_move){
            column_num = c;
            row_num = r;
        } else {
            println!("That's not a row or column on the grid. Please use a-f and 1-6 for coordinates");
            println!("");
            return false;
        }

        // Sum of ship length and row/column_num never exceeds 7
        // Checks if tiles are already taken
        if direction == "horizontal"
        && (column_num + ship_length) <= 7 {
            for i in 1..(ship_length + 1) {
                if grid[column_num + i - 2][row_num - 1] == 0 {
                    continue;
                }
                else {
                    println!("Ah! There's a ship in the way. Make sure your ships aren't stacking atop one another");
                    println!("");
                    return false;
                }
            }

            return true;
        }
        else if direction == "vertical"
        && (row_num + ship_length) <= 7 {
            for i in 1..(ship_length + 1) {
                if grid[column_num - 1][row_num + i - 2] == 0 {
                    continue;
                }
                else {
                    println!("Ah! There's a ship in the way. Make sure your ships aren't stacking atop one another");
                    println!("");
                    return false;
                }
            }

            return true;
        }
        else if ( (row_num + ship_length) <= 7 || (column_num + ship_length) <= 7 )
        &&           ( direction == "vertical" || direction == "horizontal" ) {
            println!("Your ship is literally off the charts (too long for the grid). Double check your math, unless you seriously doubt a computer's processing *lols*");
            println!("");
            return false;
        }
        else {
            println!("Check the spelling of your direction. It's either:    vertical | horizontal");
            println!("");
            return false;
        }
    }

    fn place_ships_success(
        mut grid: &mut [[usize; 6]; 6],
        p_move: &String,
        ship_len: usize)
        -> bool {
            if !(is_valid_move(&p_move, ship_len, &grid)) { return false; }

            let mut column_num: usize;
            let mut row_num: usize;

            if let Some((c, r)) = super::reuse::move_to_point_success(&p_move) {
                column_num = c;
                row_num = r;
            } else { return false; }

            let (column_num, row_num) = (column_num - 1, row_num - 1);

            let direction = &p_move[3..];

            if direction == "horizontal" {
                for i in 0..(ship_len) {
                    if grid[column_num + i][row_num] == 0 {
                          grid[column_num + i][row_num] = ship_len;
                    }
                    else {
                        panic!("Else fired in horizontal for loop @place_ships_success()");
                    }
                }
            }
            else if direction == "vertical" {
                for i in 0..ship_len {
                    if grid[column_num][row_num + i] == 0 {
                          grid[column_num][row_num + i] = ship_len;
                    }
                    else {
                        panic!("Else fired in vertical for loop @place_ships_success()");
                    }
                }
            } else { panic!("@ place_ships_success had else fire"); }

            true
        }
}

mod run_game {
    use std::io;

    pub fn player_move(
        guess_grid: &mut [[str; 6]; 6],
        target_grid: &mut [[usize; 6]; 6])
        -> () {
            let (column_num, row_num) = get_player_input_point();
            let (column, row) = (column_num - 1, row_num - 1);

            if target_grid[column][row] != 0 {
                guess_grid[column][row] = "X";

                super::reuse::print_guess_map(&guess_grid);

                update_target_grid(&mut target_grid, (column, row));

            } else {
                guess_grid[column][row] = "O";
                println!("You missed");
            }
    }

    fn get_player_input_point() -> (usize, usize) {
        let mut player_input = String::new();

        let mut column: usize;
        let mut row: usize;

        loop {
            player_input = String::new();

            io.stdin().read_line(&mut player_input)
            .expect("Didn't read line");

            if player_input.len() == 2 {
                player_input = super::reuse::slice_off_last_char(&player_input);
            } else {
                println!("That's not a valid point. Use values a-f and 1-6. Ex: a3");
                continue;
            }

            if is_valid_target(&player_move) { break; }
        }

        if let Some((c, r)) = super::reuse::move_to_point_success(&player_input) {
            return (c, r);
        } else {
            panic!("Else on if let @ get_player_input_point()");
        }
    }

    fn is_valid_target(input: &String) -> bool {
        if input.len() < 2 {
            println!("An input is at least 2 characters. Ex: a3");
            return false;
        }

        if let Some((c, r)) = super::reuse::move_to_point_success(input) {
            return true;
        } else {
            println!("That's not a value on the grid. Please use a-f and 1-6. Ex: a3");
            return false;
        }
    }

    fn update_target_grid(
        grid :&mut [[usize; 6]; 6],
        point: (usize, usize))
        -> () {
        let (c, r) = point;

        let ship_num = grid[c][r];

        if ship_num == 0 { panic!("Ship num is 0 @ update_target_grid()"); }

        grid[c][r] = 0;

        for column in grid {
            for row in column {
                if grid[column][row] == ship_num {
                    return;
                }
            }
        }

        println!("Ship {} had been sunk", ship_num);
    }
}

mod finish {
    pub fn is_win(
        p1_grid: &[[usize; 6]; 6],
        p2_grid: &[[usize; 6]; 6])
        -> bool {

        'p1_win: for column in p1_grid {
            for row in column {
                if p1_grid[column][row] != 0 {
                    break 'p1 win;
                }
            }
            if column == 5 { return true; }
        }

        'p2_win: for column in p2_grid {
            for row in column {
                if p2_grid[column][row] != 0 {
                    break 'p2_win;
                }
            }
            if column == 5 { return true; }
        }

        false
    }
}




// fn create() {
//
// }
//
// fn game_setup() -> (Grid_Tiles, Grid_Tiles) {
//     let mut p1_ship = user_place_ships();
//     let mut p2_ship = user_place_ships();
//
//     (p1_ship, p2_ship)
// }
//
// fn user_place_ships() -> Grid_Tiles {
//     let mut ship_2_place = String::new();
//     let mut ship_3_place = String::new();
//     let mut ship_4_place = String::new();
//
//     loop {
//         if !is_valid_move(ship_2_place, 2) {
//             println!("Place your 2 tile ship (X X)");
//
//             io::stdin().read_line(&mut ship_2_place)
//             .expect("Didn't read anything");
//
//             if is_valid_move(ship_2_place, 2) { continue; }
//         }
//         else if !is_valid_move(ship_3_place, 3) {
//             println!("Place your 3 tile ship (X X X)");
//
//             io::stdin().read_line(&mut ship_3_place)
//             .expect("Didn't read anything");
//
//             if is_valid_move(ship_3_place, 3) { continue; }
//         }
//         else if !is_valid_move(ship_4_place, 4) {
//             println!("Place your 4 tile ship (X X X X)");
//
//             io::stdin().read_line(&mut ship_4_place)
//             .expect("Didn't read anything");
//
//             if is_valid_move(ship_4_place, 4) { continue; }
//         }
//         else { break; }
//
//         // Will only print if the user's placement was invalid
//         println!("");
//         println!("That isn't a valid move, or has improper syntax");
//         println!("An example of valid sytax is:   a3 vertical");
//         println!("");
//     }
//
//     let place_array = [ship_2_place, ship_3_place, ship_4_place];
//
//     for ship in place_array.iter() {
//
//     }
//
//
// }
//
// fn is_valid_move(user_ship_place: String, ship_length: u8) -> bool {
//     let column = &user_ship_place[0..1];
//     let row = &user_ship_place[1..2];
//     let direction = &user_ship_place[3..];
//
//     if column == "a" || column == "b" || column == "c"
//     || column == "d" || column == "e" || column == "f"
//     && row == "1" || row == "2" || row == "3"
//     || row == "4" || row == "5" || row == "6"
//     && direction == "vertical" || direction == "horizontal" {
//         match ship_length {
//             "2" => {
//                 if direction == "vertical"
//                 && row != "6"
//                 { return true; }
//
//                 else if direction == "horizontal"
//                 && column != "f"
//                 { return true; }
//
//                 else { return false; }
//             },
//             "3" => {
//                 if direction == "vertical"
//                 && row != "6" && row != "5"
//                 { return true; }
//
//                 else if direction == "horizontal"
//                 && column != "f" && column != "e"
//                 { return true; }
//
//                 else { return false; }
//             },
//             "4" => {
//                 if direction == "vertical"
//                 && row != "6" && row != "5" && row != "4"
//                 { return true; }
//
//                 else if direction == "horizontal"
//                 && column != "f" && column != "e" && column != "d"
//                 { return true; }
//
//                 else { return false; }
//             },
//              _  => panic!("Error on ship length @is_valid_move()");,
//         }
//     }
//     else {
//         return false;
//     }
// }

//   A   B   C   D   E   F
// 1   |   |   |   |   |
// 2   |   |   |   |   |
// 3   |   |   |   |   |
// 4   |   |   |   |   |
// 5   |   |   |   |   |
// 6   |   |   |   |   |

// ship 2: (X X)
// ship 3: (X X X)
// ship 4: (X X X X)



// let mut p2_guess = GridTiles {
//     a1: false,
//     b1: false,
//     c1: false,
//     d1: false,
//     e1: false,
//     f1: false,
//     a2: false,
//     b2: false,
//     c2: false,
//     d2: false,
//     e2: false,
//     f2: false,
//     a3: false,
//     b3: false,
//     c3: false,
//     d3: false,
//     e3: false,
//     f3: false,
//     a4: false,
//     b4: false,
//     c4: false,
//     d4: false,
//     e4: false,
//     f4: false,
//     a5: false,
//     b5: false,
//     c5: false,
//     d5: false,
//     e5: false,
//     f5: false,
//     a6: false,
//     b6: false,
//     c6: false,
//     d6: false,
//     e6: false,
//     f6: false,
// };
