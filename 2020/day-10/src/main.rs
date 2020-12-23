use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut numbers: Vec<usize> = contents
        .trim_end()
        .split('\n')
        .map(|s| s.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    println!("{:?}", numbers);

    numbers.sort_unstable();

    let results: Vec<usize> = vec![];

    let diffs: (Vec<usize>, Vec<usize>) = numbers
        .iter()
        .fold((results, 0), |(mut acc, last_item), current_item| {
            let diff = current_item - last_item;
            println!(
                "diff of {:?} found between {:?} and {:?}",
                diff, current_item, last_item
            );
            if diff != 2 {
                acc.push(diff);
            }
            (acc, *current_item)
        })
        .0
        .into_iter()
        .partition(|x| *x == 1);

    println!("counts: {:?}", diffs.0.len() * (diffs.1.len() + 1));
}
