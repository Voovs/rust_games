extern crate dialoguer;
use dialoguer:: { Select };

use super::games;

pub fn select_game() {
    let mut message = Select::new();

    message.item("battle_ship");
    message.item("tic tac toe");

    match message.interact().unwrap() {
        0 => games::battle_ship::main(),
        1 => {
            use std::collections::HashMap;

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

            games::tic_tac_toe::main(true, spaces);
        },
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
