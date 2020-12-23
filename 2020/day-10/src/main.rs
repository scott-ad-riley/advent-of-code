use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn run(filename: &str) -> usize {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut numbers: Vec<usize> = contents
        .trim_end()
        .split('\n')
        .map(|s| s.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    numbers.insert(0, 0);

    numbers.sort_unstable();

    let options: HashSet<usize> = numbers.iter().cloned().collect();

    let max = numbers.iter().max().unwrap();

    let mut subtree_counts: HashMap<usize, usize> = HashMap::new();
    let mut position = (numbers.len() - 1) as i64;

    while position >= 0 {
        count_subtree(
            &numbers,
            position as usize,
            &mut subtree_counts,
            &options,
            &max,
        );
        position -= 1;
    }

    *subtree_counts.get(&0).unwrap()
}

fn get_values_from(item: usize, options: &HashSet<usize>, max: &usize) -> Vec<usize> {
    vec![item + 1, item + 2, item + 3]
        .into_iter()
        .filter(|x| x <= max && options.contains(x))
        .collect()
}

fn count_subtree(
    sorted_numbers: &Vec<usize>,
    pos: usize,
    subtree_counts: &mut HashMap<usize, usize>,
    options: &HashSet<usize>,
    max: &usize,
) {
    let target = sorted_numbers[pos];

    let possible_value_counts: Vec<usize> = get_values_from(target, &options, max)
        .iter()
        .map(|x| *subtree_counts.get(x).unwrap())
        .collect();

    let total_from_this_node: usize = possible_value_counts.iter().sum();

    subtree_counts.insert(target, total_from_this_node.max(1));
}
fn main() {
    println!("result {:?}", run("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two() {
        assert_eq!(run("input2.txt"), 8);
    }

    #[test]
    fn test_three() {
        assert_eq!(run("input3.txt"), 19208);
    }

    #[test]
    fn test_four() {
        assert_eq!(run("input4.txt"), 3);
    }

    #[test]
    fn test_five() {
        assert_eq!(run("input5.txt"), 5);
    }

    #[test]
    fn test_six() {
        assert_eq!(run("input6.txt"), 13);
    }
}
