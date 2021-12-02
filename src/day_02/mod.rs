use crate::common::get_file_lines;

pub fn run() {
    let instructions = read_instructions("src/day_02/input.txt");
    
    println!("\nPart one");
    let result = part_one(&instructions);
    println!("Answer: {}", result);

    println!("\nPart two");
    let result = part_two(&instructions);
    println!("Answer: {}", result);
}

fn part_one(instructions: &Vec<Instruction>) -> u32 {
    let mut position = 0;
    let mut depth = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => { 
                position += instruction.value; 
            },
            Direction::Up => {
                depth -= instruction.value;
            },
            Direction::Down => {
                depth += instruction.value;
            }
        }
    }

    return position * depth;
}

fn part_two(instructions: &Vec<Instruction>) -> u32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => { 
                position += instruction.value;
                depth += aim * instruction.value;
            },
            Direction::Up => {
                aim -= instruction.value;
            },
            Direction::Down => {
                aim += instruction.value;
            }
        }
    }

    return position * depth;
}

fn read_instructions(file_path: &str) -> Vec<Instruction> {
    return get_file_lines(file_path)
        .map(|line| to_instruction(&line.unwrap()))
        .collect();
}

enum Direction {
    Forward,
    Up,
    Down,
}

struct Instruction {
    direction: Direction,
    value: u32,
}

fn to_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(" ").collect();

    return Instruction {
        direction: match parts[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => Direction::Forward,
        },
        value: parts[1].parse::<u32>().unwrap()
    }
}