// #![feature(iterator_fold_self)]
#[macro_use]
extern crate lazy_static;
extern crate regex;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

type Color = String;

struct Contained {
    count: usize,
    name: Color,
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // this is a map of Color -> Color that bag contains
    let mut bags: HashMap<String, Vec<(usize, &str)>> = HashMap::new();

    // this is a map of Color -> Color that contains that bag
    let mut inverted_bags: HashMap<Color, Vec<Color>> = HashMap::new();

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

    println!("{:?}", bags);

    let mut counter = 0;

    get_contained_bag_count("shiny gold".to_string(), &bags, &mut counter);

    println!("{:?}", counter);
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

fn get_contained_bag_count(
    target_bag: String,
    bag_type_to_contents_map: &HashMap<String, Vec<(usize, &str)>>,
    acc: &mut usize,
) -> usize {
    let bags_inside_target = bag_type_to_contents_map.get(&target_bag).unwrap();

    if bags_inside_target.len() == 0 {
        println!("{:?} contains no bags (current acc:{:?})", target_bag, acc);
        return 1;
    }

    bags_inside_target
        .iter()
        .map(|&(inner_bag_count, inner_bag_type)| {
            let count = inner_bag_count
                * get_contained_bag_count(
                    inner_bag_type.to_string(),
                    bag_type_to_contents_map,
                    acc,
                );

            *acc += count;

            println!(
                "{:?} contains {:?}x{:?} bags (current total:{:?} just added:{:?})",
                target_bag, inner_bag_count, inner_bag_type, acc, count
            );

            count
        })
        .inspect(|x| println!("summing: {:?}", x))
        .sum()
}
