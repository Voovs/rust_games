use std::io;
use std::collections::HashMap;

pub fn main(is_xplayer_move: bool, mut spaces: HashMap<&str, &str>) {
    let mut user_move = String::new();

    io::stdin().read_line(&mut user_move)
    .expect("Didn't read anything");

    let user_move: &str = &user_move[0..2];

    if is_legal_move(user_move, &spaces) {
        update_board(user_move, &mut spaces, is_xplayer_move);
        print_board(&spaces);

        let (player_win, board_full) = is_player_win_is_full_board(&spaces);

        if player_win {
            match is_xplayer_move {
                true => println!("Well done player X. You have won the game"),
                false => println!("Well done player O. You have won the game"),
            }
            return;
        }
        else if board_full {
            println!("Looks like nobody won. Let's go for a another round");

            let mut spaces: HashMap<&str, &str> = HashMap::new();
            {
                spaces.insert("a1", " ");
                spaces.insert("b1", " ");
                spaces.insert("c1", " ");
                spaces.insert("a2", " ");
                spaces.insert("b2", " ");
                spaces.insert("c2", " ");
                spaces.insert("a3", " ");
                spaces.insert("b3", " ");
                spaces.insert("c3", " ");
            }

            main(true, spaces);
        }
        else {
            main(!is_xplayer_move, spaces);
        }
    } else {
        println!("Sorry, {} is occupied or else unavailable. Please go again", user_move);
        main(is_xplayer_move, spaces);
    }
}

fn is_legal_move(user_move: &str, spaces: &HashMap<&str, &str>) -> bool {
    if let Some(b) = spaces.get(user_move) {
        let b = *b;

        if b == " " {
            return true;

        } else if b == "x" || b == "o" {
            println!("{} is already taken by an {} tile. Please try again", user_move, b);
            return false;

        } else {
            println!("Something went wrong @ is_legal_move: {}", b);
            return false;
        }

    }
    else {
        return false;
    }
}

fn update_board(user_move: &str, spaces: &mut HashMap<&str, &str>, is_xplayer_move: bool) {
    if is_xplayer_move {
        match user_move {
            "a1" => spaces.insert("a1", "x"),
            "b1" => spaces.insert("b1", "x"),
            "c1" => spaces.insert("c1", "x"),
            "a2" => spaces.insert("a2", "x"),
            "b2" => spaces.insert("b2", "x"),
            "c2" => spaces.insert("c2", "x"),
            "a3" => spaces.insert("a3", "x"),
            "b3" => spaces.insert("b3", "x"),
            "c3" => spaces.insert("c3", "x"),
            _ => spaces.insert("", " "),
        };
    } else if !is_xplayer_move {
        match user_move {
            "a1" => spaces.insert("a1", "o"),
            "b1" => spaces.insert("b1", "o"),
            "c1" => spaces.insert("c1", "o"),
            "a2" => spaces.insert("a2", "o"),
            "b2" => spaces.insert("b2", "o"),
            "c2" => spaces.insert("c2", "o"),
            "a3" => spaces.insert("a3", "o"),
            "b3" => spaces.insert("b3", "o"),
            "c3" => spaces.insert("c3", "o"),
            _ => spaces.insert("", " "),
        };
    } else {
        println!("Error @ update_board()");
    }
}

fn print_board(spaces: &HashMap<&str, &str>) {
    let [a1, b1, c1, a2, b2, c2, a3, b3, c3] = deconst_tiles(spaces);

    println!("");
    println!("      A   B   C");
    println!("   1  {} | {} | {}", a1, b1, c1);
    println!("   2  {} | {} | {}", a2, b2, c2);
    println!("   3  {} | {} | {}", a3, b3, c3);
    println!("");

}

fn is_player_win_is_full_board(spaces: &HashMap<&str, &str>) -> (bool, bool) {
    let [a1, b1, c1, a2, b2, c2, a3, b3, c3] = deconst_tiles(spaces);

    // println!("{}", a1);
    // println!("{}", b1);
    // println!("{}", c1);
    // println!("{}", a2);
    // println!("{}", b2);
    // println!("{}", c2);
    // println!("{}", a3);
    // println!("{}", b3);
    // println!("{}", c3);

    let blk_srg = " ".to_string();

    if (b1 != blk_srg) && ((a1 == b1) && ((a1 == c1) && (b1 == c1))) // rows
    || (b2 != blk_srg) && ((a2 == b2) && ((a2 == c2) && (b2 == c2)))
    || (b3 != blk_srg) && ((a3 == b3) && ((a3 == c3) && (b3 == c3)))
    || (a2 != blk_srg) && ((a1 == a2) && ((a1 == a3) && (a2 == a3))) // columns
    || (b2 != blk_srg) && ((b1 == b2) && ((b1 == b3) && (b2 == b3)))
    || (c2 != blk_srg) && ((c1 == c2) && ((c1 == c3) && (c2 == c3)))
    || (b2 != blk_srg) && ((a1 == b2) && ((a1 == c3) && (b2 == c3))) // diagonal
    || (b2 != blk_srg) && ((c1 == b2) && ((c1 == a3) && (b2 == a3))) { return (true, false); }

    else if a1 != " "
         && b1 != " "
         && c1 != " "
         && a2 != " "
         && b2 != " "
         && c2 != " "
         && a3 != " "
         && b3 != " "
         && c3 != " "{ return (false, true); }

    else { return (false, false); }

}

fn deconst_tiles(spaces: &HashMap<&str, &str>) -> [String; 9] {
    let mut all_tiles: [String; 9] = ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];

    let array_tile_spaces = ["a1", "b1", "c1", "a2", "b2", "c2", "a3", "b3", "c3"];

    let mut iter_count = 0;

    for tile in array_tile_spaces.iter() {
        if spaces.contains_key(tile) {
            let s = *spaces.get(tile).unwrap();

            all_tiles[iter_count] = s.to_string();

        } else {
            all_tiles[iter_count] = " ".to_owned();
        }

        iter_count += 1;
    }

    all_tiles
}
