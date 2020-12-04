#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let result = contents
        .split("\n\n")
        .map(|row| parse_row(row))
        .map(|passport| is_valid(&passport))
        .filter(|value| *value)
        .count();

    println!("{:?}", result);
}

fn is_valid(passport: &HashMap<String, String>) -> bool {
    return vec![
        validate_byr(passport.get("byr")),
        validate_iyr(passport.get("iyr")),
        validate_eyr(passport.get("eyr")),
        validate_hgt(passport.get("hgt")),
        validate_hcl(passport.get("hcl")),
        validate_ecl(passport.get("ecl")),
        validate_pid(passport.get("pid")),
    ]
    .iter()
    .map(|opt| opt.unwrap_or(false))
    .all(|value| value);
}

fn validate_byr(value: Option<&String>) -> Option<bool> {
    Some(between(value?, 1920, 2002))
}

fn validate_iyr(value: Option<&String>) -> Option<bool> {
    Some(between(value?, 2010, 2020))
}

fn validate_eyr(value: Option<&String>) -> Option<bool> {
    Some(between(value?, 2020, 2030))
}

fn validate_hgt(value: Option<&String>) -> Option<bool> {
    lazy_static! {
        static ref IN_REGEX: Regex = Regex::new(r"^([0-9]{1,})in").unwrap();
    }

    if IN_REGEX.is_match(value?) {
        let amount = IN_REGEX.captures_iter(value?).nth(0)?.get(1)?;
        return Some(between(amount.as_str(), 59, 76));
    }

    lazy_static! {
        static ref CM_REGEX: Regex = Regex::new(r"^([0-9]{1,})cm").unwrap();
    }

    if CM_REGEX.is_match(value?) {
        let amount = CM_REGEX.captures_iter(value?).nth(0)?.get(1)?;

        return Some(between(amount.as_str(), 150, 193));
    }

    None
}

fn validate_hcl(value: Option<&String>) -> Option<bool> {
    lazy_static! {
        static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }

    return Some(HCL_REGEX.is_match(value?));
}

fn validate_ecl(value: Option<&String>) -> Option<bool> {
    Some(vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value?))
}

fn validate_pid(value: Option<&String>) -> Option<bool> {
    lazy_static! {
        static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    Some(PID_REGEX.is_match(value?))
}

fn between(value: &str, min: usize, max: usize) -> bool {
    let maybe_num = value.parse();

    if maybe_num.is_err() {
        return false;
    }

    let num: usize = maybe_num.unwrap();

    return num >= min && num <= max;
}

fn parse_row(row: &str) -> HashMap<String, String> {
    let cleaned_row = row.replace("\n", " ");

    cleaned_row
        .split(" ")
        .fold(HashMap::new(), |mut acc, item| {
            let mut item = item.split(":");
            let maybe_key = item.next();
            let maybe_value = item.next();
            match (maybe_key, maybe_value) {
                (Some(key), Some(value)) => {
                    acc.insert(key.replace("\n", ""), value.replace("\n", " "));
                }
                (_, _) => (),
            }
            acc
        })
}
