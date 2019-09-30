use std::io;
use typename::TypeName;

struct tileOnGrid {
    o_upper: &str,
    o_lower: &str,
    x_upper: &str,
    x_lower: &str,
}

fn main() {
    let mut (pO_score, pX_score, is_pX_turn) = (0, 0, true);
    let winning_score = setup_and_welcome();
    let mut tile_positions = tileOnGrid {
            o_upper: "2A",
            o_lower: "4A",
            x_upper: "1D",
            x_lower: "3D",
        };

    while pO_score != winning_score && pX_score != winning_score {
        let mut player_move: String = "az";//Get player's move

        if is_legal_move(player_move) {
            print_map(player_move);

            if is_player_point {
                match is_pX_turn {
                    true => pX_score += 1,
                    false => pO_score += 1,
                }
            }

            is_pX_turn = !is_pX_turn;
        } else {
            println!("That move could not be completed. Please replay your turn");
        }
    }

    if pO_score == winning_score && pX_score != winning_score {
        println!("Player O wins with a score of {}", winning_score);
    } else if pX_score == winning_score && pX_score != winning_score{
        println!("Player X wins with a score of {}", winning_score);
    } else {
        println!("Sorry, however something errored out");
        println!("Try restarting the game file with Ctrl + C,");
    }
}

fn setup_and_welcome () -> u8 {
    println!("Welcome to ---");
    println!("Let's start with some simple setup");
    println!("The rules are pretty simple, try to get your tiles to the opposing end row. Each tile in the end row gives you a point!");


    println!("Now choose how many points your game will go up to (max is 10):");
    let winning_score = set_winning_score();

    while !( 0 < winning_score <= 11){
        println!("Please try setting the score again. Remember the highest score can be 10 and lowest 1. The score must be an integer")
        winning_score = set_winning_score();
    }


}

fn set_winning_score () -> u8 {
    let winning_score = ;//user input
    if 0 < winning_score < 11 && TypeId::of::winning_score == TypeId::of::<String>() {
        println!("First to {} wins!", winning_score);
        return winning_score;
    } else {
        println!("That value doesn't work. Please make sure your winning score is 1-10 and is an integer:");
        return set_winning_score();
    }
}

fn is_legal_move (move) -> bool {

}

fn update_tile_pos (move) {

}

fn is_player_point (tile_positions) -> bool {

}

fn print_map (tile_positions) {

}



//             Rules of --- with graphics
//
//    Starting layout
//             -----------------
//             |   | O |   | o |
//             |   |   |   |   |
//             |   |   |   |   |
//             | X |   | x |   |
//             -----------------
//
//    Possible moves for O (shown with o)
//             -----------------
//             | o | o | o |   |
//             | o | O | o |   | (O can move 1 tile in
//             | o | o | o |   |  any direction)
//             |   |   |   |   |
//             -----------------
//
//    Blocking- Can't move on enemy tile's space
//             -----------------
//             | o | o | o |   |
//             | o | O | o |   | (O cannot move on the
//             | o | X | o |   |  same tile as the X)
//             |   |   |   |   |
//             -----------------
//
//    Move exception- Last row of enemy line diagonal
//             -----------------
//             |   |   |   |   |
//             |   | o | o | o |
//             |   | o | O | o |
//             |   |   | X |   | (O can't move diagonally
//             -----------------  beside X, in last row)
//
//    Attack move- One of a team on both x and y axis
//             -----------------
//             |   |   |   |   | (Since there's an "x"
//             |   |   |   |   |  tile in the same row
//             |   | X | O |   |  as O, and an "x" in
//             |   |   | x |   |  the same column, O
//             -----------------  will respawn)
//
//    Gaining a point- get a tile into opposing end row
//             -----------------
//             |   |   |   |   | (O spawned in top row)
//             |   |   |   |   | (When O moves into the
//             |   |   |   |   |  bottom row, tile O
//             | O |   | x |   |  will respawn and
//             -----------------  player O gets +1 point)
//
//    Respawning- Top left for O and bottom right for X
//             -----------------
//             | O |   |   |   | (Tile O respawns in the
//             |   |   |   |   |  first spot available
//             |   | X |   |   |  from the top left going
//             |   |   | x |   |  right)
//             -----------------
//
//    Winning- Achieve the winning number of points
//             Default is 2
