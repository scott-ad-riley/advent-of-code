use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    /*
      let rows
    */

    let trees: Vec<Vec<bool>> = contents
        .split("\n")
        .map(|row| parse_row(row))
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .collect();

    let stop_at = trees.len();
    let mut depth = 0;
    let mut tree_count = 0;
    let forest = Forest::new(trees);

    while depth < stop_at {
        if forest.has_tree(depth * 3, depth) {
            tree_count += 1;
        }
        depth += 1;
    }

    println!("{:?}", tree_count)
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<bool>>,
    row_width: usize,
}

impl Forest {
    pub fn new(trees: Vec<Vec<bool>>) -> Self {
        let row_width = trees.iter().nth(1).unwrap().len();
        Self { trees, row_width }
    }

    pub fn has_tree(&self, horizontal: usize, depth: usize) -> bool {
        let row = self.trees.iter().nth(depth);
        if row.is_none() {
            panic!("went past the last row")
        }
        let tree = row.unwrap().iter().nth(horizontal);
        match tree {
            Some(true) => true,
            Some(false) => false,
            None => self.has_tree(horizontal - self.row_width, depth),
        }
    }
}

fn parse_row(row: &str) -> Option<Vec<bool>> {
    if !row.contains("#") || !row.contains(".") {
        return None;
    }
    Some(
        row.chars()
            .map(|char| match char {
                '#' => true,
                '.' => false,
                _ => panic!("unknown character"),
            })
            .collect(),
    )
}
