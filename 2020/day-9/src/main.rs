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
        if let None = find_sum(target, &items) {
            println!("No match found for {:?}", target);
            find_contiguous(target, &numbers);
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

fn find_contiguous(target: &usize, numbers: &Vec<usize>) -> Option<usize> {
    let mut items_ptr_to_start_from: usize = 0;
    let mut items_ptr_to_end_at: usize = 1;

    while &numbers[items_ptr_to_end_at] < target {
        let slice = &numbers[items_ptr_to_start_from..items_ptr_to_end_at];
        let sum: usize = slice.into_iter().sum();
        if sum < *target {
            items_ptr_to_end_at += 1;
        } else if sum > *target {
            items_ptr_to_start_from += 1;
            items_ptr_to_end_at = items_ptr_to_start_from + 1;
        } else {
            println!("slice: {:?}", slice);

            return None;
        }
    }

    panic!("unable to find a pair of sums")
}
