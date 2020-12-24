use std::{fmt, fs};
mod seat;
use seat::*;
fn main() {
    run("input.txt");
}

fn run(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut seating = SeatingState::new(contents.trim_end().split('\n').collect());

    let mut tracker = (100000, 200000);
    let mut next_count = 1000000;

    while tracker.0 != tracker.1 {
        seating = seating.next_plan();
        next_count = seating.count_occupied();
        tracker = (tracker.1, next_count);
    }

    println!("Occupied: {}", next_count);
}

#[derive(Debug)]
struct SeatingState {
    rows: Vec<Vec<Seat>>,
}

impl fmt::Display for SeatingState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows.iter() {
            for seat in row {
                write!(f, "{}", seat)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl SeatingState {
    fn new(input_rows: Vec<&str>) -> Self {
        let rows: Vec<Vec<Seat>> = input_rows
            .into_iter()
            .map(|row| {
                row.chars()
                    .map(|x| match x {
                        'L' => Seat::Empty,
                        '.' => Seat::Floor,
                        '#' => Seat::Occupied,
                        _ => panic!("unrecognised seat type"),
                    })
                    .collect()
            })
            .collect();
        Self { rows }
    }

    fn print(&self) {
        println!("{}", self)
    }

    fn next_plan(&self) -> Self {
        let (start_x, start_y): (usize, usize) = (0, 0);
        let mut rows: Vec<Vec<Seat>> = Vec::new();
        let (max_x, max_y): (usize, usize) = (self.rows.first().unwrap().len(), self.rows.len());

        for y in start_y..max_y {
            let mut row: Vec<Seat> = vec![];
            for x in start_x..max_x {
                row.push(self.determine_next(x, y))
            }
            rows.push(row)
        }

        Self { rows }
    }

    fn determine_next(&self, x: usize, y: usize) -> Seat {
        let current = &self.rows[y][x];
        let mut occupied_count: usize = 0;

        for (step_x, step_y) in relevant_seat_steps() {
            let mut step_count = 1;

            let mut found_seat = false;

            while !found_seat {
                let new_x: isize = x as isize + (step_x * step_count);
                let new_y: isize = y as isize + (step_y * step_count);

                let outside_bounds = new_x < 0 || new_y < 0;
                if outside_bounds {
                    found_seat = true;
                } else {
                    // isize -> usize is the potentially lossy one but we're safe becase we know
                    // that new_y and new_x are both zero or above (i.e. safe to convert to usize)
                    let target_seat = self
                        .rows
                        .get(new_y as usize)
                        .and_then(|row| row.get(new_x as usize));

                    if target_seat.is_none() {
                        found_seat = true
                    } else {
                        match target_seat.unwrap() {
                            Seat::Floor => {}
                            Seat::Occupied => {
                                occupied_count += 1;
                                found_seat = true
                            }
                            Seat::Empty => found_seat = true,
                        }
                    }
                }

                step_count += 1;
            }
        }

        match (current, occupied_count >= 5, occupied_count == 0) {
            (Seat::Floor, _, _) => Seat::Floor,
            (Seat::Occupied, true, _) => Seat::Empty,
            (Seat::Occupied, false, _) => Seat::Occupied,
            (Seat::Empty, _, true) => Seat::Occupied,
            (Seat::Empty, _, false) => Seat::Empty,
        }
    }

    fn count_occupied(&self) -> usize {
        self.rows.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|x| matches!(x, Seat::Occupied)).count()
        })
    }
}

fn relevant_seat_steps() -> Vec<(isize, isize)> {
    vec![
        (-1, 0),  // left
        (1, 0),   // right
        (0, -1),  // above
        (0, 1),   // below
        (-1, -1), // top left
        (1, -1),  // top right
        (-1, 1),  // bottom left
        (1, 1),   // bottom right
    ]
}

fn adjacents_and_diagonals(x: usize, y: usize) -> Vec<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    vec![
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter(|(x, y)| *x >= 0 && *y >= 0)
    .map(|(x, y)| (x as usize, y as usize))
    .collect()
}
