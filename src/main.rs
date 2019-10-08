use std::io;
use typename::TypeName;

struct tileOnGrid {
    o_upper: &str,
    o_lower: &str,
    x_upper: &str,
    x_lower: &str,
}

fn main() {
    let mut (pO_score, pX_score, is_pX_turn, ) = (0, 0, true);

    const WINNING_SCORE = setup_and_welcome();

    let mut tile_positions = tileOnGrid {
            o_upper: "2A",
            o_lower: "4A",
            x_upper: "1D",
            x_lower: "3D",
        };

    while pO_score != WINNING_SCORE && pX_score != WINNING_SCORE {
        let mut player_move = String::new();

        io::stdin().read_line(&mut player_move)
        .expect("Player's move caused an error");

        if is_legal_move(player_move) {
            let mut (pO_score, pX_score) = check_for_point(&tile_positions, pO_score, pX_score, is_pX_turn);

            if

            tile_positions = update_tile_pos(player_move, &mut tile_positions);

            print_map(&tile_positions);

            is_pX_turn = !is_pX_turn;
        } else {
            println!("That move could not be completed. Please replay your turn");
        }
    }

     if announce_win_and_reset(pO_score, pX_score) {
         main();
     }
}

/* Instructions and WINNING_SCORE
 * Return: u8 (WINNING_SCORE)
 */
fn setup_and_welcome () -> u8 {
    println!("Welcome to ---");
    println!("Let's start with some simple setup");
    println!("The rules are pretty simple, try to get your tiles to the opposing end row. Each tile in the end row gives you a point!");
    println!("Now choose how many points your game will go up to (max is 10):");

    const WINNING_SCORE = set_winning_score();

    return WINNING_SCORE;
}

/* User input for winning_score
 * Return: u8 (winning_score)
 */
fn set_winning_score () -> u8 {
    let mut winning_score = String::new();

    io::stdin().read_line(&mut winning_score)
    .expect("The input step failed");

    const WINNING_SCORE: u8 = winning_score.parse().unwrap();

    if 0 < WINNING_SCORE < 11 { // && TypeId::of::winning_score == TypeId::of::<u8>()
        println!("First to {} wins!", WINNING_SCORE);

        return WINNING_SCORE;
    } else {
        println!("That value doesn't work. Please make sure your winning score is 1-10 and is an integer:");

        return set_winning_score();
    }
}

fn is_legal_move (move) -> bool {

}

/* Updates and returns score
 * Return: () (score of player O, score of player X)
 */
fn check_for_point (tile_positions, pO_score, pX_score, is_pX_turn) -> () {
    match is_pX_turn {
        true => {
            if tile_positions.x_upper[0] == "A" {
                pX_score += 1

            } else if tile_positions.x_lower[0] == "A"{

            }
        },
        false => {
            if tile_positions.o_upper[0] == "D" || tile_positions.o_lower[0] == "D" {
                pO_score += 1
            }
        },
    }
    return (pO_score, pX_score)
}

fn update_tile_pos (move, tile_positions) -> struct {

}


fn print_map (tile_positions) {

}

/* Prints win and returns true if restart is chosen
 * Return: bool (true for game restart)
 */
fn announce_win_and_reset (pO_score, pX_score) -> bool {
    if pO_score == winning_score && pX_score != winning_score {
        println!("Player O wins with a score of {}", winning_score);
    } else if pX_score == winning_score && pX_score != winning_score{
        println!("Player X wins with a score of {}", winning_score);
    } else {
        println!("Sorry, however something errored out");
        println!("Try restarting the game file with Ctrl + C");
    }
    // delay
    let play_again: bool = false;

    // Let player select yes or no

    return play_again
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
