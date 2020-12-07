use itertools::Itertools;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut result = contents
        .split("\n")
        .map(|row| seat_id(row))
        .sorted()
        .peekable();

    let mut similar_seats: Vec<(usize, usize)> = Vec::new();
    let mut cloned_result = result.clone();
    let mut last_seat_id: usize = *cloned_result.peek().unwrap();

    while let Some(seat_id) = result.next() {
        if (seat_id - last_seat_id) > 1 {
            similar_seats.push((last_seat_id, seat_id));
        }
        last_seat_id = seat_id.clone();
    }

    println!("{:?}", similar_seats);
}

fn seat_id(row: &str) -> usize {
    if row.len() < 3 {
        return 0;
    }

    let row_slice = &row[..7];
    let column_slice = &row[7..10];
    let row = seat_row(row_slice);
    let column = seat_column(column_slice);

    row * 8 + column
}

fn seat_row(position: &str) -> usize {
    let replaced = position.replace("B", "1").replace("F", "0");
    usize::from_str_radix(&replaced, 2).unwrap()
}

fn seat_column(position: &str) -> usize {
    let replaced = position.replace("R", "1").replace("L", "0");
    usize::from_str_radix(&replaced, 2).unwrap()
}
