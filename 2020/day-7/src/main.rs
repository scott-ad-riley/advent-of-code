// #![feature(iterator_fold_self)]
#[macro_use]
extern crate lazy_static;
extern crate regex;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // loop through the file, produce a hashmap of bag_type: Vec<(usize,child_bag_type)>

    let mut bags: HashMap<String, Vec<(usize, &str)>> = HashMap::new();
    contents
        .split("\n")
        .filter(|x| x.len() > 3)
        .map(|row| {
            let mut items = row.split("contain");
            let bag_type = extract_bag_type(items.next().unwrap());
            let contents: Vec<(usize, &str)> = extract_contents(items.next().unwrap());
            bags.insert(bag_type.clone().to_string(), contents);

            bag_type
        })
        .collect_vec();

    let count = get_container_bags(vec!["shiny gold".to_string()], &bags, &mut HashSet::new());

    println!("{:?}", count);
}

fn extract_bag_type(item: &str) -> String {
    item.trim_end().clone().replace(" bags", "")
}

fn extract_contents(item: &str) -> Vec<(usize, &str)> {
    lazy_static! {
        static ref CONTENTS_REGEX: Regex = Regex::new(r"([0-9]{1})\s(\w*\s\w*)\sbags?\.?").unwrap();
    }
    item.trim_start()
        .split(",")
        .filter(|x| !x.contains("no other bags"))
        .map(|x| {
            CONTENTS_REGEX
                .captures_iter(x)
                .map(|capture| {
                    (
                        capture.get(1).unwrap().as_str().parse().unwrap(),
                        capture.get(2).unwrap().as_str(),
                    )
                })
                .nth(0)
                .unwrap()
        })
        .collect()
}

fn get_container_bags(
    target_bags: Vec<String>,
    bag_type_to_contents_map: &HashMap<String, Vec<(usize, &str)>>,
    acc: &mut HashSet<String>,
) -> usize {
    let bags_containing_target_bags: Vec<(&String, &Vec<(usize, &str)>)> = bag_type_to_contents_map
        .iter()
        .filter(|(_, contents)| {
            contents.iter().any(|(_count, each_bag_type)| {
                target_bags
                    .iter()
                    .any(|target_bag| target_bag == each_bag_type)
            })
        })
        .collect();

    if bags_containing_target_bags.len() == 0 {
        return acc.len();
    }

    let next_bags: Vec<String> = bags_containing_target_bags
        .iter()
        .map(|(a, _)| (*a).clone())
        .collect();
    next_bags.iter().for_each(|x| {
        acc.insert(x.to_string());
    });
    return get_container_bags(next_bags, bag_type_to_contents_map, acc);
}
