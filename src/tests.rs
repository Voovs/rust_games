enum Team {
    Name(String),
    ErrorNum(usize),
    Guess(Vec<char>),
}

impl Team {
    fn get_name(self) -> String {
        if let Team::Name(n) = self { n } else { panic!("Not a team's name!") }
    }
    fn get_num(self) -> usize {
        if let Team::ErrorNum(n) = self { n } else { panic!("Not a team's error number!") }
    }
    fn get_vec_item(&self, index: usize) -> char {
        if let Team::Guess(v) = &self { v[index] } else { panic!("Not a vector") }
    }
}

#[test]
fn test_name() {
    let team_name = Team::Name(String::from("Lovely Humans"));

    assert_eq!(team_name.get_name(), "Lovely Humans".to_string());
}

#[test]
fn test_error_num() {
    let error_num = Team::ErrorNum(4);

    assert_eq!(4, error_num.get_num());
}

#[test]
fn test_guess_vec() {
    let my_vec = vec![' ', 'a', 't'];

    let team_guess = Team::Guess(my_vec);

    assert_eq!('a', team_guess.get_vec_item(1));
}

#[test]
fn test_regex() {
    use regex::Regex;

    let my_string = String::from("Hey");
    let raw_string = r"Hey";

    let re_letter = Regex::new(r"[a-z]").unwrap();
    let re_string = Regex::new(&my_string).unwrap();

    assert!(re_letter.is_match(&my_string));

    assert!(re_string.is_match(&my_string));
}
