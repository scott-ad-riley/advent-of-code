use std::{collections::VecDeque, fs};

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let numbers: Vec<usize> = contents
        .trim_end()
        .split("\n")
        .map(|s| s.parse::<usize>().clone())
        .filter_map(Result::ok)
        .collect();

    let mut items: VecDeque<usize> = numbers.clone().into_iter().take(25).collect();

    for target in &numbers[25..] {
        match find_sum(target, &items) {
            Some(_) => {}
            None => println!("No match found for {:?}", target),
        }
        items.pop_front();
        items.push_back(*target);
    }
}

fn find_sum(target: &usize, items: &VecDeque<usize>) -> Option<(usize, usize)> {
    for outer_item in items {
        for inner_item in items {
            if outer_item != inner_item && outer_item + inner_item == *target {
                return Some((*outer_item, *inner_item));
            }
        }
    }

    None
}
