use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq)]
enum Action {
    Acc,
    Jump,
    Noop,
}
#[derive(Debug)]
struct Instruction {
    action: Action,
    value: isize,
    line_number: isize,
}

impl Instruction {
    fn new(line: &str, line_number: isize) -> Self {
        let mut parsed = line.split(" ");
        let left = parsed.next().unwrap();
        let right: isize = parsed.next().unwrap().parse().unwrap();
        let action = match left {
            "acc" => Action::Acc,
            "jmp" => Action::Jump,
            "nop" => Action::Noop,
            _ => panic!("unrecognised value"),
        };

        Self {
            action,
            line_number,
            value: right,
        }
    }
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut instructions = Vec::new();
    let mut line_number = 0;
    let mut modifiable: Vec<isize> = vec![];
    let mut to_modify_ptr: usize = 0;

    for line in contents.trim_end().split("\n") {
        instructions.push(Instruction::new(line, line_number));
        if !line.contains("acc") {
            modifiable.push(line_number);
        }
        line_number += 1;
    }

    let mut accummulator = 0;
    let mut instruction_pointer: isize = 0;
    let mut seen_instructions: HashSet<isize> = HashSet::new();

    while (instruction_pointer + 1) < instructions.len() as isize {
        if instruction_pointer < 0 {
            panic!("instruction pointer got below zero")
        }

        let unsigned_line_number: usize = instruction_pointer as usize;
        let instruction = &instructions[unsigned_line_number];
        let is_modifiable = instruction.action != Action::Acc;
        let seen_already = seen_instructions.contains(&instruction_pointer);
        seen_instructions.insert(instruction_pointer);

        if !is_modifiable && !seen_already {
            match instruction.action {
                Action::Acc => {
                    accummulator += instruction.value;
                    instruction_pointer += 1;
                }
                Action::Jump => instruction_pointer += instruction.value,
                Action::Noop => instruction_pointer += 1,
            };
        } else {
            if seen_already {
                instruction_pointer = 0;
                accummulator = 0;
                seen_instructions.drain();
                to_modify_ptr += 1;
            } else {
                let instruction_ptr_to_modify = modifiable[to_modify_ptr];
                let should_modify = instruction_ptr_to_modify == instruction_pointer;

                if should_modify {
                    println!(
                        "running modified node #{:?} (instruction: {:?})",
                        to_modify_ptr, instruction_ptr_to_modify
                    );
                }

                match (&instruction.action, should_modify) {
                    (Action::Acc, _) => panic!("trying to modify an Action::Acc"),
                    (Action::Jump, true) | (Action::Noop, false) => instruction_pointer += 1,
                    (Action::Noop, true) | (Action::Jump, false) => {
                        instruction_pointer += instruction.value
                    }
                }
            }
        }
    }

    println!("{:?}", accummulator);
}
