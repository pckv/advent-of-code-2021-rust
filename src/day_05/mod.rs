use crate::common::get_file_lines;
use crate::day_05::line::Line;
use crate::day_05::position::Position;
use std::collections::HashMap;
use std::collections::HashSet;

mod line;
mod position;

pub fn run() {
    let lines: Vec<Line> = get_file_lines("src/day_05/input.txt")
        .map(|raw_line| {
            let line: Line = raw_line.unwrap().parse().unwrap();

            // Normalize so that the line always goes towards the right or down
            // and prioritize rising x for diagonals
            if line.start.x > line.end.x
                || (line.start.y > line.end.y && line.start.x == line.end.x)
            {
                return Line {
                    start: line.end,
                    end: line.start,
                };
            } else {
                return line;
            }
        })
        .collect();

    println!("\nPart one");
    let result = part_one(&lines);
    println!("Answer: {}", result);

    println!("\nPart two");
    let result = part_two(&lines);
    println!("Answer: {}", result);
}

fn part_one(lines: &Vec<Line>) -> u32 {
    return count_overlapping_positions(&lines, get_linear_positions);
}

fn part_two(lines: &Vec<Line>) -> u32 {
    return count_overlapping_positions(&lines, get_linear_and_diagonal_positions);
}

fn count_overlapping_positions(
    lines: &Vec<Line>,
    position_iterator: fn(&Line) -> Option<Vec<Position>>,
) -> u32 {
    let mut position_count: HashMap<Position, u32> = HashMap::new();

    for line in lines {
        if let Some(new_positions) = position_iterator(&line) {
            for new_position in new_positions {
                *position_count.entry(new_position).or_insert(0) += 1;
            }
        }
    }

    return position_count
        .values()
        .fold(0, |acc, count| if *count > 1 { acc + 1 } else { acc });
}

// Gets positions from vertical and horizontal lines only
fn get_linear_positions(line: &Line) -> Option<Vec<Position>> {
    // Horizontal line
    if line.start.x == line.end.x {
        return Some(
            (line.start.y..line.end.y + 1)
                .map(|y| Position {
                    x: line.start.x,
                    y: y,
                })
                .collect(),
        );
    }
    // Vertical line
    if line.start.y == line.end.y {
        return Some(
            (line.start.x..line.end.x + 1)
                .map(|x| Position {
                    x: x,
                    y: line.start.y,
                })
                .collect(),
        );
    }

    return None;
}

// Also gets 45 degree angled positions
fn get_linear_and_diagonal_positions(line: &Line) -> Option<Vec<Position>> {
    if let Some(positions) = get_linear_positions(&line) {
        return Some(positions);
    }

    // Must be a diagonal
    // There are two different kinds of diagonal:
    //   0,0 -> 3,3
    //   0,3 -> 3,0

    return Some(
        (0..(line.end.x - line.start.x) + 1)
            .map(|i| Position {
                x: line.start.x + i,
                y: if line.start.y < line.end.y {
                    line.start.y + i
                } else {
                    line.start.y - i
                },
            })
            .collect(),
    );
}
