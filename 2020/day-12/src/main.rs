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
        direction: 90,
    };

    for instruction in instructions.iter() {
        position.update(instruction);
        println!("now at {:?}", position);
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
    direction: isize,
}

impl ShipPosition {
    fn update(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(value) => {
                let (new_x, new_y) = move_x_y(self.direction, *value, (self.x, self.y));
                println!("moved by {} in direction {}", value, self.direction);
                self.x = new_x;
                self.y = new_y;
            }
            Instruction::Left(value) => {
                self.direction = (self.direction - *value as isize) % 360;
            }
            Instruction::Right(value) => {
                self.direction = (self.direction + *value as isize) % 360;
            }
            Instruction::North(value) => {
                self.y -= value;
            }
            Instruction::South(value) => {
                self.y += value;
            }
            Instruction::East(value) => {
                self.x += value;
            }
            Instruction::West(value) => {
                self.x -= value;
            }
        }
    }
}

fn move_x_y(direction: isize, value: isize, current: (isize, isize)) -> (isize, isize) {
    let (x, y) = current;
    match direction {
        -90 | 270 => (x - value, y),
        90 | -270 => (x + value, y),
        180 | -180 => (x, y + value),
        0 => (x, y - value),
        _ => panic!("unrecognised direction"),
    }
}
