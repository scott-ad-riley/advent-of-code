// #![feature(iterator_fold_self)]
#[macro_use]
extern crate lazy_static;
extern crate regex;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

type Color = String;
type Contains = HashMap<Color, Vec<Contained>>;
type ContainedIn = HashMap<Color, HashSet<Color>>;

#[derive(Debug)]
struct Contained {
    count: usize,
    color: Color,
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // this is a map of Color -> Contained where the key is the container, and the value is a Vec of what's contained inside it
    let mut bags: Contains = HashMap::new();

    // this is a map of Color -> Set<Color> where the key is a child bag, and the value is a Set of of which bags contain it
    let mut inverted_bags: ContainedIn = HashMap::new();

    for line in contents.trim_end().split("\n") {
        let mut items = line.split("contain");
        let bag_type = extract_bag_type(items.next().unwrap());
        let contents: Vec<Contained> = extract_contents(items.next().unwrap());
        bags.insert(bag_type.clone().to_string(), contents);
    }

    for (bag, outer_contents) in bags.iter() {
        for contents in outer_contents {
            let entry = inverted_bags
                .entry(contents.color.to_string())
                .or_insert(HashSet::new());
            entry.insert(bag.to_string());
        }
    }

    // let part1_answer = transitive_closure(&inverted_bags, &"shiny gold".to_string()).len();

    // println!("{:?}", part1_answer);

    let part2_answer = walk_graph_for_contained_bag_count(&bags);

    println!("{:?}", part2_answer);
}

fn transitive_closure(inverted_bags: &ContainedIn, start: &Color) -> HashSet<Color> {
    let mut valid_containers = HashSet::new();
    calculate_transitive_closure_step(inverted_bags, &mut valid_containers, vec![start]);

    valid_containers.remove(start);
    valid_containers
}

fn calculate_transitive_closure_step(
    inverted_bags: &ContainedIn,
    acc: &mut HashSet<Color>,
    target: Vec<&Color>,
) {
    for color in target.iter() {
        // check we haven't already found the containers for this bag
        if !acc.contains(*color) {
            // we want to find all the places that lead to color
            acc.insert(color.to_string());
            if let Some(new_containers) = inverted_bags.get(*color) {
                calculate_transitive_closure_step(
                    inverted_bags,
                    acc,
                    new_containers.iter().collect_vec(),
                )
            }
        }
    }
}

fn walk_graph_for_contained_bag_count(bags: &Contains) -> usize {
    let mut stack = bags
        .get("shiny gold")
        .unwrap()
        .iter()
        .flat_map(|bag| to_n_strings(bag))
        .collect_vec();

    let mut count: usize = 0;

    while let Some(color) = stack.pop() {
        count += 1;

        if let Some(all_contained) = bags.get(&color) {
            for contained in all_contained {
                stack.append(&mut to_n_strings(&contained))
            }
        }
    }

    count
}

fn to_n_strings(bag: &Contained) -> Vec<Color> {
    vec![bag.color.to_string(); bag.count]
}

fn extract_bag_type(item: &str) -> String {
    item.trim_end().clone().replace(" bags", "")
}

fn extract_contents(item: &str) -> Vec<Contained> {
    lazy_static! {
        static ref CONTENTS_REGEX: Regex = Regex::new(r"([0-9]{1})\s(\w*\s\w*)\sbags?\.?").unwrap();
    }
    item.trim_start()
        .split(",")
        .filter(|x| !x.contains("no other bags"))
        .map(|x| {
            if let Some(capture) = CONTENTS_REGEX.captures_iter(x).next() {
                if let (Some(count), Some(color)) = (capture.get(1), capture.get(2)) {
                    let contained = Contained {
                        color: color.as_str().to_string(),
                        count: count.as_str().parse::<usize>().unwrap(),
                    };

                    contained
                } else {
                    panic!("Couldn't extract the parsed values from the line")
                }
            } else {
                panic!("regex didn't match the line")
            }
        })
        .collect()
}
