use regex::Regex;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let items = contents
        .split("\n")
        .map(|row| parse_row(row))
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .filter(|parsed_row| parsed_row.is_valid())
        .count();

    println!("{:?}", items)
}

struct ParsedInputRow<'a> {
    min: usize,
    max: usize,
    char: char,
    password: &'a str,
}

impl<'a> ParsedInputRow<'a> {
    pub fn is_valid(&self) -> bool {
        let first_character_matches = self.password.as_bytes()[self.min - 1] as char == self.char;
        let second_character_matches = self.password.as_bytes()[self.max - 1] as char == self.char;
        return first_character_matches ^ second_character_matches;
    }
}

fn parse_row(raw: &str) -> Option<ParsedInputRow> {
    let regex = Regex::new(r"(.{1,2})-(.{1,2})\s(.):\s(.*)").unwrap();

    let mut captures = regex.captures_iter(raw).map(|x| ParsedInputRow {
        min: x.get(1).unwrap().as_str().parse().unwrap(),
        max: x.get(2).unwrap().as_str().parse().unwrap(),
        char: x.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        password: x.get(4).unwrap().as_str(),
    });

    captures.next()
}
