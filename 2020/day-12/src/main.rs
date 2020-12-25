use std::fs;

fn main() {
    run("input.txt");
}

fn run(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let instructions: Vec<Instruction> = contents
        .trim_end()
        .split('\n')
        .map(|row| Instruction::parse(row))
        .collect();

    let mut position = ShipPosition {
        x: 0,
        y: 0,
        waypoint_x: 10,
        waypoint_y: -1,
    };

    for instruction in instructions.iter() {
        position.update(instruction);
    }

    println!("{:?}", position);
}

#[derive(Debug)]
enum Instruction {
    Forward(isize),
    Left(isize),
    Right(isize),
    North(isize),
    South(isize),
    East(isize),
    West(isize),
}

impl Instruction {
    fn parse(row: &str) -> Self {
        let variant = &row[0..1];
        let value = row[1..].parse::<isize>().unwrap();

        match variant {
            "F" => Instruction::Forward(value),
            "L" => Instruction::Left(value),
            "R" => Instruction::Right(value),
            "N" => Instruction::North(value),
            "S" => Instruction::South(value),
            "E" => Instruction::East(value),
            "W" => Instruction::West(value),
            _ => panic!("unrecognised instruction"),
        }
    }
}

#[derive(Debug)]
struct ShipPosition {
    x: isize,
    y: isize,
    waypoint_x: isize,
    waypoint_y: isize,
}

impl ShipPosition {
    fn update(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(value) => {
                self.x += self.waypoint_x * value;
                self.y += self.waypoint_y * value;
            }
            Instruction::Left(value) => {
                let (new_x, new_y) = rotate_clockwise(value, self.waypoint_x, self.waypoint_y);
                self.waypoint_x = new_x;
                self.waypoint_y = new_y;
            }
            Instruction::Right(value) => {
                let (new_x, new_y) =
                    rotate_counterclockwise(value, self.waypoint_x, self.waypoint_y);
                self.waypoint_x = new_x;
                self.waypoint_y = new_y;
            }
            Instruction::North(value) => {
                self.waypoint_y -= value;
            }
            Instruction::South(value) => {
                self.waypoint_y += value;
            }
            Instruction::East(value) => {
                self.waypoint_x += value;
            }
            Instruction::West(value) => {
                self.waypoint_x -= value;
            }
        }
    }
}

fn rotate_clockwise(amount: &isize, x: isize, y: isize) -> (isize, isize) {
    match amount {
        270 => (-y, x),
        90 => (y, -x),
        180 => (-x, -y),
        0 => (x, y),
        _ => panic!("unrecognised direction"),
    }
}

fn rotate_counterclockwise(amount: &isize, x: isize, y: isize) -> (isize, isize) {
    match amount {
        270 => (y, -x),
        90 => (-y, x),
        180 => (-x, -y),
        0 => (x, y),
        _ => panic!("unrecognised direction"),
    }
}
