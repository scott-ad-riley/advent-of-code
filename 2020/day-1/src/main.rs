use std::{collections::HashMap, fs};

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let items = contents
        .split("\n")
        .map(|row| row.parse())
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap());

    let mut sums_map: HashMap<u32, (u32, u32)> = HashMap::new();

    items.clone().for_each(|number| {
        items.clone().for_each(|other_number| {
            if number != other_number && number + other_number < 2020 {
                sums_map.insert(number + other_number, (number, other_number));
            }

            ()
        })
    });

    let result: Vec<&(u32, u32)> = items
        .clone()
        .filter_map(|number| sums_map.get(&(2020 - number)))
        .collect();

    println!("{:?}", result);
}
