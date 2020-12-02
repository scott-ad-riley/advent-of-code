use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let items: Vec<(u32, u32)> = contents
        .split("\n")
        .map(|row| row.parse::<u32>())
        .filter(|item| item.is_ok())
        .map(|item| {
            let unwrapped = item.unwrap();
            (unwrapped, 2020 - unwrapped)
        })
        .collect();

    let result = items
        .iter()
        .find(|(_original, diff)| {
            let result_match = items.iter().find(|&&each_item| diff == &each_item.0);

            if result_match.is_some() {
                true
            } else {
                false
            }
        })
        .unwrap();

    println!("{:?}", result)
}
