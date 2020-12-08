#![feature(iterator_fold_self)]
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::{collections::HashSet, fs};

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let result: usize = contents
        .split("\n\n")
        .map(|row| GroupAnswers::new(&row))
        .map(|group_answered_set| group_answered_set.build_all_answered_yes_set().len())
        .sum();

    println!("{:?}", result)
}

#[derive(Debug)]
struct GroupAnswers<'a> {
    pub answer_sets: Vec<HashSet<&'a str>>,
}

impl<'a> GroupAnswers<'a> {
    fn new(raw_input: &'a str) -> Self {
        Self {
            answer_sets: raw_input
                .split("\n")
                .map(|row| build_answered_set(row))
                .collect(),
        }
    }

    fn build_all_answered_yes_set(&self) -> HashSet<&'a str> {
        self.answer_sets
            .clone()
            .into_iter()
            .filter(|x| !x.is_empty())
            .fold_first(|group_answered_set, individual_answered_set| {
                group_answered_set
                    .intersection(&individual_answered_set)
                    .cloned()
                    .collect()
            })
            .unwrap()
    }
}

fn build_answered_set(row: &str) -> HashSet<&str> {
    lazy_static! {
        static ref LETTER_REGEX: Regex = Regex::new(r"([a-z])").unwrap();
    }

    LETTER_REGEX
        .captures_iter(&row)
        .fold(HashSet::new(), |mut acc, cap| {
            acc.insert(cap.get(1).unwrap().as_str());

            acc
        })
}
