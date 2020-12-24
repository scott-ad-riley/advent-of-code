use std::fs;

fn main() {
    run("input.txt");
}

fn run(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
}
