use std::io;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn main() {
  let mut p1_ships = [[0; 6]; 6];
  let mut p2_ships = [[0; 6]; 6];
  let mut p1_guesses = [[' '; 6]; 6];
  let mut p2_guesses = [[' '; 6]; 6];

  setup::ready_ships_and_intro(&mut p1_ships, &mut p2_ships);

  let mut is_p1_turn = true;

  while !finish::is_win(&p1_ships, &p2_ships) {
      if is_p1_turn {
          println!("");
          println!("Player 1, it's your turn to shoot");
          println!("Your previous guesses:");

          reuse::print_guess_map(&p1_guesses);

          run_game::player_move(&mut p1_guesses, &mut p2_ships);

          reuse::pause_for_handoff("Hit enter and pass to player 2");
      }
      else {
          println!("");
          println!("Player 2, it's your turn to shoot");
          println!("Your previous guesses:");

          reuse::print_guess_map(&p2_guesses);

          run_game::player_move(&mut p2_guesses, &mut p1_ships);

          reuse::pause_for_handoff("Hit enter and pass to player 1");
      }
      is_p1_turn = !is_p1_turn;
  }

  finish::announce_win(&p1_ships, &p2_ships, &p1_guesses, &p2_guesses);
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
    pub fn print_guess_map(grid: &[[char; 6]; 6]) {
        use std::io;

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
    pub fn input_to_point_success(p_move: &String) -> Option<(usize, usize)> {
        // Prevents slice panic on insufficient user input string length
        if p_move.len() < 2 { return None; }

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

        Some((column_num - 1, row_num - 1))
    }
    pub fn slice_off_last_char(string: &String) -> String {
          let string = &string[..(string.len() - 1)].to_string();

          string.to_string();

          // let string = *string;

          string.to_owned()
        }
    pub fn pause_for_handoff(message: &str) {
        use std::io;

        let mut hit_enter = String::new();

        println!("{}", message);

        io::stdin().read_line(&mut hit_enter)
        .expect("Didn't read anything");

        print!("\x1B[2J");

        println!(">>> Only hit enter if it's your turn <<<");

        io::stdin().read_line(&mut hit_enter)
        .expect("Didn't read anything");

        print!("\x1B[2J");
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
                println!("Valid syntax is (a-f)(1-6) (v or h). Ex: a3 v");
                println!("");

                user_place_ships(&mut p1_ships);

                super::reuse::print_ship_map(&p1_ships);

                println!("Player 1, your ships have been placed");
                super::reuse::pause_for_handoff("Press enter and hand off to player 2, to continue");
            }
            {
                // print!("{}[2J", 27 as char); // Clears cmd screen
                println!("It's player 2's turn. Get ready to place your ships");
                println!("");

                user_place_ships(&mut p2_ships);

                super::reuse::print_ship_map(&p2_ships);

                println!("Player 2, your ships have been placed");
                super::reuse::pause_for_handoff("Press enter and hand off to player 1, to continue");
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
        if p_move.len() < 4 {
            println!("That's not proper syntax for a move. Ex:       a3 v");
            println!("");
            return false;
        }

        let direction = &p_move[3..];

        let mut column_num: usize;
        let mut row_num: usize;

        if let Some((c, r)) = super::reuse::input_to_point_success(p_move){
            column_num = c + 1;
            row_num = r + 1;
        } else {
            println!("That's not a row or column on the grid. Please use a-f and 1-6 for coordinates");
            println!("");
            return false;
        }

        // Sum of ship length and row/column_num never exceeds 7
        // Checks if tiles are already taken
        if direction == "h"
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
        else if direction == "v"
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
        &&                  ( direction == "v" || direction == "h" ) {
            println!("Your ship is literally off the charts (too long for the grid). Double check your math, unless you seriously doubt a computer's processing *lols*");
            println!("");
            return false;
        }
        else {
            println!("Check the spelling of your direction. It's either:    v (for vertical) OR h (for horizontal)");
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

            if let Some((c, r)) = super::reuse::input_to_point_success(&p_move) {
                column_num = c;
                row_num = r;
            } else { return false; }

            let direction = &p_move[3..];

            if direction == "h" {
                for i in 0..(ship_len) {
                    if grid[column_num + i][row_num] == 0 {
                          grid[column_num + i][row_num] = ship_len;
                    }
                    else {
                        panic!("Else fired in horizontal for loop @place_ships_success()");
                    }
                }
            }
            else if direction == "v" {
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
        guess_grid: &mut [[char; 6]; 6],
        mut target_grid: &mut [[usize; 6]; 6])
        -> () {
        let (column, row) = get_player_input_point(&guess_grid);

        if target_grid[column][row] != 0 {
            guess_grid[column][row] = 'X';

            println!("Your new guesses:");

            super::reuse::print_guess_map(&guess_grid);

            update_target_grid(&mut target_grid, (column, row));

        } else {
            guess_grid[column][row] = 'O';

            println!("You missed");
        }
    }

    #[allow(unused_assignments)]
    fn get_player_input_point(g_grid: &[[char; 6]; 6]) -> (usize, usize) {
        let mut player_input = String::new();

        loop {
            player_input = String::new();

            io::stdin().read_line(&mut player_input)
            .expect("Didn't read line");

            if player_input.len() == 3 {
                player_input = super::reuse::slice_off_last_char(&player_input);
            } else {
                println!("An input is at least 2 characters. Ex: a3");
                continue;
            }

            if is_valid_target(&player_input, &g_grid) { break; }
        }

        if let Some((c, r)) = super::reuse::input_to_point_success(&player_input) {
            return (c, r);
        } else {
            panic!("Else on if let @ get_player_input_point()");
        }
    }

    fn is_valid_target(input: &String, g_grid: &[[char; 6]; 6]) -> bool {
        if input.len() < 2 {
            println!("An input is at least 2 characters. Ex: a3");
            return false;
        }

        if let Some((c, r)) = super::reuse::input_to_point_success(input) {
            if g_grid[c][r] == ' ' {
                return true;
            }
            else {
                println!("You've already guesses at that tile. Repeating it won't get you anything");
                return false;
            }
        } else {
            println!("That's not a tile on the field. Please use a-f and 1-6. Ex: a3");
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

        for column in 0..6 {
            for row in 0..6 {
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

        'p1_win: for column in 0..6 {
            for row in 0..6 {
                if p1_grid[column][row] != 0 {
                    break 'p1_win;
                }
            }
            if column == 5 { return true; }
        }

        'p2_win: for column in 0..6 {
            for row in 0..6 {
                if p2_grid[column][row] != 0 {
                    break 'p2_win;
                }
            }
            if column == 5 { return true; }
        }

        false
    }

    pub fn announce_win(
        p1_ships: &[[usize; 6]; 6],
        p2_ships: &[[usize; 6]; 6],
        p1_guesses: &[[char; 6]; 6],
        p2_guesses: &[[char; 6]; 6])
        -> () {
        let mut win_player_num = ' ';
        let mut lose_player_num = ' ';

        'win_p_num: for column in 0..6 {
            for row in 0..6 {
                if p1_ships[column][row] != 0 {
                    println!("P_1 win on column: {} | row: {}", column, row);
                    win_player_num = '1';
                    lose_player_num = '2';
                    break 'win_p_num;
                }
            }
            if column == 5 {
                println!("P_2 win on column: {}", column);
                win_player_num = '2';
                lose_player_num = '1';
            }
        }


        println!("Well done player {}! Your skills, with absolutely no luck, have allowed you to shoot down all player {} ships. You can now relax, after these potent graphics have left nothing to the imagination", win_player_num, lose_player_num);
        pause_no_handoff();

        println!("Here are your final fields");
        println!("Player 1's surviving ships:");
        super::reuse::print_ship_map(&p1_ships);
        pause_no_handoff();

        println!("Player 2's surviving ships");
        super::reuse::print_ship_map(&p2_ships);
        pause_no_handoff();

        println!("Player 1's guesses on the field");
        super::reuse::print_guess_map(&p1_guesses);
        pause_no_handoff();

        println!("Player 2's guesses on the field");
        super::reuse::print_guess_map(&p2_guesses);
        println!("");

        println!("Thanks for playing!!!");
    }

    fn pause_no_handoff() {
        use std::io;

        let mut hit_enter = String::new();

        println!("");

        println!("Press enter to continue:");

        io::stdin().read_line(&mut hit_enter)
        .expect("Didn't read anything");
    }
}
