extern crate dialoguer;
use dialoguer:: { Select };

use super::games;

pub fn select_game() {
    let mut message = Select::new();

    message.item("battle_ship");
    message.item("tic tac toe");

    match message.interact().unwrap() {
        0 => games::battle_ship::main(),
        1 => games::tic_tac_toe::main(),
        _ => panic!("match in menu.rs @select_game() fired"),
    }
}

pub fn is_play_again() -> bool {
    let mut play_again = Select::new();

    println!("");
    println!("Would you like to play another game?");

    play_again.item("Yes please!");
    play_again.item("Not a chance. Calling that a game shames games");

    match play_again.interact().unwrap() {
        0 => true,
        1 => false,
        _ => {
            panic!("match in menu.rs @is_play_again() fired");
            return false;
        },
    }
}
