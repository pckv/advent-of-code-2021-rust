use crate::common::get_file_lines;

pub fn run() {
    let instructions = read_instructions("src/day_02/input.txt");
    println!("\nPart one");
    let result = part_one(&instructions);
    println!("Answer: {}", result);

    println!("\nPart one (functional)");
    let result = part_one_functional(&instructions);
    println!("Answer: {}", result);

    println!("\nPart two");
    let result = part_two(&instructions);
    println!("Answer: {}", result);

    println!("\nPart two (functional)");
    let result = part_two_functional(&instructions);
    println!("Answer: {}", result);
}

fn part_one(instructions: &Vec<Instruction>) -> u32 {
    let mut position = 0;
    let mut depth = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => {
                position += instruction.value;
            }
            Direction::Up => {
                depth -= instruction.value;
            }
            Direction::Down => {
                depth += instruction.value;
            }
        }
    }

    return position * depth;
}

fn part_one_functional(instructions: &Vec<Instruction>) -> u32 {
    let (position, depth) =
        instructions
            .into_iter()
            .fold((0, 0), |(position, depth), instruction| {
                match instruction.direction {
                    Direction::Forward => (position + instruction.value, depth),
                    Direction::Up => (position, depth - instruction.value),
                    Direction::Down => (position, depth + instruction.value),
                }
            });
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
            }
            Direction::Up => {
                aim -= instruction.value;
            }
            Direction::Down => {
                aim += instruction.value;
            }
        }
    }

    return position * depth;
}

fn part_two_functional(instructions: &Vec<Instruction>) -> u32 {
    let (position, depth, _) =
        instructions
            .into_iter()
            .fold(
                (0, 0, 0),
                |(position, depth, aim), instruction| match instruction.direction {
                    Direction::Forward => (
                        position + instruction.value,
                        depth + aim * instruction.value,
                        aim,
                    ),
                    Direction::Up => (position, depth, aim - instruction.value),
                    Direction::Down => (position, depth, aim + instruction.value),
                },
            );

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

fn parse_direction(direction: &str) -> Result<Direction, &'static str> {
    return match direction {
        "forward" => Ok(Direction::Forward),
        "up" => Ok(Direction::Up),
        "down" => Ok(Direction::Down),
        _ => Err("invalid direction"),
    };
}

struct Instruction {
    direction: Direction,
    value: u32,
}

fn to_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(" ").collect();

    return Instruction {
        direction: parse_direction(parts[0]).unwrap(),
        value: parts[1].parse::<u32>().unwrap(),
    };
}
