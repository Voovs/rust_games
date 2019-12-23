#[allow(non_snake_case)]
pub fn main() {
    let mut grid = [[0; 3]; 3];

    let mut is_pX_turn = true;

    'run_game: loop {
        player_move(&mut grid, is_pX_turn);

        if is_win(&grid) {
            annouce_win(is_pX_turn, &grid);
            break;
        }
        else {
            is_pX_turn = !is_pX_turn;
        }
    }
}

#[allow(non_snake_case)]
#[allow(unused_assignments)]
fn player_move(mut grid: &mut [[usize; 3]; 3], is_pX_turn: bool) {
    use std::io;

    let mut user_move = String::new();

    print_grid(&grid);

    if is_pX_turn {
        println!("Player X, it is your turn");
    } else {
        println!("Player O, it is your turn");
    }

    'play_makes_move: loop {
        user_move = String::new();

        io::stdin().read_line(&mut user_move)
        .expect("Didn't read anything");

        if is_valid_move(&grid, &user_move) { break; }
        else { println!(""); }
    }

    update_grid(&mut grid, &user_move, is_pX_turn);
}

fn is_win(grid: &[[usize; 3]; 3]) -> bool {
    let g = grid;

    for c in 0..3 {
        if g[c][0] == g[c][1] && g[c][1] == g[c][2] && g[c][1] != 0 {
            return true;
        }
    }
    for r in 0..3 {
        if g[0][r] == g[1][r] && g[1][r] == g[2][r] && g[1][r] != 0 {
            return true;
        }
    }

    if ( g[0][0] == g[1][1] && g[1][1] == g[2][2]
    ||   g[0][2] == g[1][1] && g[1][1] == g[2][0] )
    && g[1][1] != 0 {
        return true;
    }

    false
}

fn is_valid_move(
    grid: &[[usize; 3]; 3],
    user_move: &String)
    -> bool {
    if user_move.len() < 2 {
        println!("That's too short to be a move. Synax is [a-c][1-3]. Ex: a3");
        return false; }

    let column = &user_move[0..1];
    let row = &user_move[1..2];

    if ( column == "a" || column == "b" || column == "c" )
    && (    row == "1" ||    row == "2" || row == "3" ) {
        let (column_num, row_num) = move_to_point(&user_move);

        if grid[column_num][row_num] == 0 {
            return true;
        }
        else {
            println!("Ah! That tile is already taken. Please go again to a different tile");
        }
    }

    println!("That's not a proper coordinate. Synax is [a-c][1-3]. Ex: a3");
    false
}

fn move_to_point(user_move: &String) -> (usize, usize) {
    let column = match &user_move[0..1] {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        _ => { panic!("_ fired on match @ move_to_point() in tic_tac_toe.rs on column"); }
    };
    let row = match &user_move[1..2] {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        _ => { panic!("_ fired on match @ move_to_point() in tic_tac_toe.rs on row"); }
    };

    (column, row)
}

#[allow(non_snake_case)]
fn update_grid(
    mut grid: &mut [[usize; 3]; 3],
    user_move: &String,
    is_pX_turn: bool)
    -> () {
    let (c, r) = move_to_point(&user_move);

    let tile_str = match is_pX_turn{
        true => 1,
        false => 2,
    };

    grid[c][r] = tile_str;
}

fn print_grid(grid: &[[usize; 3]; 3]) {
    let mut g = [[" "; 6]; 6];

    'num_to_display_tiles: for column in 0..3 {
        for row in 0..3 {
            g[column][row] = match grid[column][row] {
                0 => " ",
                1 => "X",
                2 => "O",
                _ => { panic!("In tic_tac_toe.rs @print_grid() match _ fired"); }
            }
        }
    }

    println!("");
    println!("       A   B   C");
    println!("    1  {} | {} | {}", g[0][0], g[1][0], g[2][0]);
    println!("    2  {} | {} | {}", g[0][1], g[1][1], g[2][1]);
    println!("    3  {} | {} | {}", g[0][2], g[1][2], g[2][2]);
    println!("");

}

#[allow(non_snake_case)]
fn annouce_win(is_pX_turn: bool, grid: &[[usize; 3]; 3]) {
    let winning_player = match is_pX_turn {
        true => "X",
        false => "O",
    };

    println!("");
    println!("Well done player {}, you have won!!!", winning_player);
    println!("Revel in your victory");
    println!("");
    println!("Here's the final grid:");
    println!("");

    print_grid(&grid);
}
