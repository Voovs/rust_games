use std::io;
use regex::Regex;


pub struct Team {
    name: String,
    error_num: usize,
    found_chars: Vec<char>,
    prev_guess: Vec<String>
}

impl Team {
    fn print_board(&self, max_err: usize) {
        println!("");
        print!("    ");
        println!("{}:", &self.name);
        println!("");

        let mut char_num = 0;

        print!("    ");

        for character in self.found_chars.iter() {
            print!("{}", character);
            print!(" ");
            char_num += 1;
        }

        println!("");
        print!("    ");

        for underscore in 0..char_num {
            print!("_");
            print!(" ");
        }

        println!("");
        println!("");
        print!("    ");
        println!("Errors: {} / {}", &self.error_num, max_err);

        println!("");
    }
}

pub fn main() {
    prompt::intro();

    let num_of_teams: usize = setup::team_num();
    let secret_word: String = setup::secret_word();
    let max_err: usize = setup::max_err();

    let mut teams: Vec<Team> = setup::teams(num_of_teams, secret_word.len());

    let mut round_num = 0;

    if teams.len() == 1 {
        'play_one_team_game: loop {
            if in_game::is_win(&teams, &secret_word) {break 'play_one_team_game; }
            else if teams[0].error_num < max_err {
                in_game::play_turn(&secret_word, &mut teams[0], max_err);
            }
            else {
                println!("");
                println!("");
                println!("Sorry {}, yet you have lost", teams[0].name);
                println!("");
                println!("Better luck next time");
                println!("");
            }
        }
    }
    else if teams.len() > 1 {
        let mut team_num: usize = 0;

        'team_turn: loop {
            if teams[team_num].error_num >= max_err { continue; }

            in_game::play_turn(&secret_word, &mut teams[team_num], max_err);

            // continue to next not eliminated team
            let mut i: usize = team_num + 1;

            loop {
                if i == teams.len() {
                    if in_game::is_win(&teams, &secret_word) { break 'team_turn; }

                    for n in 0..teams.len() {
                        if teams[n].error_num < max_err {
                            prompt::trans_teams(&teams, n, max_err);

                            round_num += 1;
                            team_num = n;

                            continue 'team_turn;
                        }
                    }

                    prompt::everyone_lost(&secret_word);
                    return;
                }
                else if i < teams.len() && teams[i].error_num < max_err {
                    prompt::trans_teams(&teams, i, max_err);
                    team_num = i;
                    continue 'team_turn;
                }

                i += 1;
            }
        }
    }
    else { panic!("Else fired @ main() in hang_man.rs"); }

    prompt::win(&teams, max_err);
}

mod prompt {
    use std::io; use super::Team;

    pub fn intro() {
        println!("Welcome to hangman (English only)");
        println!("Start by designating one person to choose the word");
        println!("Sort the everyone else into as many teams as 9 teams");
        println!("");
        println!("Each team will get to guess a letter, one at a time");
        println!("Don't worry about who's going first. All the teams will have an equal number of guesses");
        println!("");
        println!("Your objective is to be the first team to guess the word");
        println!("");
    }
    pub fn team_eliminated(team_name: &String) {
        println!("");
        println!("team {} has been eliminated", team_name);
        println!("Having gotten so many letters wrong, you now dress the gallows");
        println!("");
    }
    pub fn trans_teams(teams: &Vec<Team>, next_team_num: usize, max_err: usize) {
        let next_team_name = &teams[next_team_num].name;

        let mut hit_enter = String::new();

        println!("Hit enter and pass the turn to: {}", next_team_name);

        io::stdin().read_line(&mut hit_enter)
        .expect("Didn't read anything");

        print!("\x1B[2J");

        println!(">>> Only {} should hit enter <<<", next_team_name);

        io::stdin().read_line(&mut hit_enter)
        .expect("Didn't read anything");

        print!("\x1B[2J");
    }
    pub fn win(teams: &Vec<Team>, max_err: usize) {
        let mut teams_won: Vec<usize> = Vec::new();

        'check_who_won: for team_num in 0..teams.len() {
            if &teams[team_num].error_num < &max_err {

                for c in teams[team_num].found_chars.iter() {

                    if *c == ' ' { continue 'check_who_won; }
                }
                teams_won.push(team_num);
            }
        }

        if teams_won.len() == 1 {
            let team_num = teams_won[0];
            let team_name = &teams[team_num].name;

            print!("\x1B[2J");
            print!("\x1B[2J");
            if &team_name[(team_name.len() - 1)..] == "s" {
                println!("{} have won the game!!!", team_name);
            }
            else {
                println!("{} has won the game!!!", team_name);
            }
            println!("");
            println!("");
            teams[team_num].print_board(max_err);
            println!("");
            println!("Congratulations!!!");
            println!("");
        }
        else if teams_won.len() > 1 {
            let secret_word = teams[teams_won[0]].found_chars.iter().collect::<String>();

            print!("\x1B[2J");
            print!("\x1B[2J");

            for team_num in teams_won.iter() {
                let team_name = &teams[*team_num].name;

                if &team_name[(team_name.len() - 1)..] == "s" {
                    println!("{} have won the game!!!", team_name);
                }
                else {
                    println!("{} has won the game!!!", team_name);
                }
                println!("");
            }
            println!("Congratulations winners!!!");
            println!("They all got the word {} on the same round!", secret_word);
            println!("");
            println!("");
        }
        else { panic!("Else fired @ is_win() in hang_man.rs"); }
    }
    pub fn everyone_lost(secret_word: &String) {
        println!("Oh wow... So all the teams have lost...");
        println!("");
        println!("That kind of sucks. Everyone exceeded the maximum number of errors");
        println!("");
        println!("The secret word was {}, btw", secret_word);
        println!("");
        println!("Consider choosing a higher allowance of errors, next time");
        println!("");
    }
}

mod setup {
    use std::io; use regex::Regex; use super::Team;

    pub fn team_num() -> usize {
        let mut input_num_of_teams = String::new();
        println!("How many teams are competing in this round?");

        'get_team_num: loop {
            input_num_of_teams = String::new();

            io::stdin().read_line(&mut input_num_of_teams)
            .expect("Didn't read an input");

            input_num_of_teams = super::slice_off_last_char(&input_num_of_teams);

            let re_number = Regex::new(r"[0-9]").unwrap();

            for n in 0..input_num_of_teams.len() {
                let c = input_num_of_teams.chars().nth(n).unwrap().to_string();

                if re_number.is_match(&c) { continue; }
                else {
                    println!("That's not a number between 1 and 10");
                    continue 'get_team_num;
                }
            }

            if let Ok(n) = input_num_of_teams.parse::<usize>() {
                if n >= 1 { return n; }
                else {
                    println!("You need to have at least one team");
                    continue 'get_team_num;
                }
            }
            else {
                panic!("Else fired @team_num() in hang_man.rs");
            }
        }
    }
    pub fn secret_word() -> String {
        let mut word = String::new();

        println!("What's the secret word?");

        'get_secret_word: loop {
            word = String::new();

            io::stdin().read_line(&mut word)
            .expect("Didn't read anything");

            println!("");

            if is_valid_secret_word(&word) {
                word = super::slice_off_last_char(&word);
                break;
            }
        }

        print!("\x1B[2J");
        print!("\x1B[2J");
        print!("\x1B[2J");
        println!("The secret word has been set");

        word
    }
    pub fn max_err() -> usize {
        let mut num = String::new();

        loop {
            println!("");
            println!("How many letters can the teams get wrong, before being eliminated?");

            num = String::new();

            io::stdin().read_line(&mut num)
            .expect("Didn't read anything");

            if let Ok(number) = super::slice_off_last_char(&num).parse::<usize>() {
                if number < 26 && number != 0 {
                    return number;
                }
                else {
                    println!("That number isn't within a reasonable range");
                    println!("Make sure your number is between 1 and 25 inclusive");
                }
            }
            else {
                println!("That's not a number. Try again");
            }
        }
    }
    pub fn teams(num_of_teams: usize, word_len: usize) -> Vec<Team> {
        let mut teams: Vec<Team> = Vec::new();

        for team_num in 0..num_of_teams {
            let mut team = Team {
                name: String::new(),
                error_num: 0,
                found_chars: vec![' '; word_len],
                prev_guess: Vec::new(),
            };

            let mut team_name = String::new();

            'get_team_name: loop {
                team_name = String::new();

                println!("");
                println!("What will team {}'s name be?", team_num + 1);

                io::stdin().read_line(&mut team_name)
                .expect("Didn't read the line");

                team_name = super::slice_off_last_char(&team_name);

                for otr_team in teams.iter() {
                    if otr_team.name == team_name {
                        println!("That name has already been taken");
                        continue 'get_team_name;
                    }
                }

                if team_name.len() > 0 {
                    team.name = team_name;
                    println!("Welcome {} to the ring!!!", team.name);
                    println!("");
                    break 'get_team_name;
                }
                else {
                    println!("Make sure your team's name is at least 1 letter long. Type in a new name:");
                }
            }

            teams.push(team);
        }

        println!("All teams have been named:");
        println!("");

        'announce: for team in teams.iter() {
            println!("- {}!", team.name);
        }
        println!("");

        {
            let mut hit_enter = String::new();

            println!("Give it to {} and hit enter to begin", teams[0].name);

            io::stdin().read_line(&mut hit_enter)
            .expect("Didn't read line");
        }

        teams
    }
    fn is_valid_secret_word(word: &String) -> bool {
        let re_letter =       Regex::new(r"[a-z]").unwrap();
        let re_upper_letter = Regex::new(r"[A-Z]").unwrap();

        if word.len() < 4 {
            println!("That word is too short. Make it at least 4 letters long");
            return false;
        }

        let word = super::slice_off_last_char(&word);

        'only_letters_in_word: for n in 0..word.len() {
            let letter = word.chars().nth(n).unwrap().to_string();

            if re_letter.is_match(&letter) {
                continue;
            }
            else if re_upper_letter.is_match(&letter) {
                println!("Please use only lower case letters");
                return false;
            }
            else {
                println!("You can only use letters a-z, with no accents or punctuation");
                return false;
            }
        }

        true
    }
}

mod in_game {
    use std::io; use regex::Regex; use super::Team;

    pub fn play_turn(
        secret_word: &String,
        mut team: &mut Team,
        max_err: usize)
        {
        team.print_board(max_err);
        println!("");
        println!("");

        println!("{}, it's your turn to guess a letter:", &team.name);

        let guess = get_user_guess(&team.prev_guess);

        team.prev_guess.push(guess.clone());

        if !is_correct_guess(&guess, &secret_word) { team.error_num += 1; }
        else { reveal_letter(&mut team, &secret_word, &guess); }

        team.print_board(max_err);
    }
    fn get_user_guess(
        prev_guess: &Vec<String>)
        -> String {
        let re_letter =       Regex::new(r"[a-z]").unwrap();
        let re_upper_letter = Regex::new(r"[A-Z]").unwrap();

        let mut guess = String::new();

        'get_guess: loop {
            guess = String::new();

            io::stdin().read_line(&mut guess)
            .expect("Didn't read line");

            guess = super::slice_off_last_char(&guess);


            if guess.len() == 1 && re_letter.is_match(&guess){
                for g in prev_guess.iter() {
                    if g == &guess {
                        println!("You've already guessed this letter");
                        continue 'get_guess;
                    }
                }
                return guess;
            }
            else if re_upper_letter.is_match(&guess) {
                println!("Only guess lower case letters");
            }
            else if !re_letter.is_match(&guess) {
                println!("Only use letters a-z in your guess");
            }
            else if guess.len() < 1 {
                println!("Guess one letter. Spaces don't count");
            }
            else if guess.len() > 1 {
                println!("You can only guess one letter at a time");
            }
            else {
                panic!("Else fired @get_user_guess() in hang_man.rs");
            }
        }

    }
    fn is_correct_guess(
        guess: &String,
        secret_word: &String)
        -> bool {
        let re_guess = Regex::new(guess).unwrap();

        if re_guess.is_match(&secret_word) { return true; }
        else {
            println!("{} is not in the word", guess);
            return false;
        }
    }
    fn reveal_letter(
        team: &mut Team,
        secret_word: &String,
        guess: &String)
        {
        for n in 0..secret_word.len() {
            if *guess == secret_word.chars().nth(n).unwrap().to_string() {
                let guess_char = guess.chars().nth(0).unwrap();

                team.found_chars[n] = guess_char;
            }
        }
    }
    pub fn is_win(
        teams: &Vec<Team>,
        secret_word: &String)
        -> bool {
        'check_for_win: for team_num in 0..teams.len() {
            let curr_word: String = teams[team_num].found_chars.iter().collect();

            if &curr_word == secret_word { return true; }
        }

        false
    }
}

fn slice_off_last_char(string: &String) -> String {
    if string.len() < 2 { return "".to_string(); }

    let string = &string[..(string.len() - 1)].to_string();

    string.to_string()
}
