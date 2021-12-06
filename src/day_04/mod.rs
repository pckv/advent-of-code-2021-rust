use crate::common::get_file_lines;
use std::str::from_utf8;

type Board = Vec<Vec<u8>>;
const BOARD_WIDTH: usize = 5;
const BOARD_HEIGTH: usize = 5;

pub fn run() {
    let mut lines = get_file_lines("src/day_04/input.txt").map(|line| line.unwrap());

    let numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    while lines.next().is_some() {
        boards.push(
            (0..BOARD_HEIGTH)
                .map(|_| {
                    lines
                        .next()
                        .unwrap()
                        .as_bytes()
                        .chunks(3)
                        .map(|chunk| from_utf8(chunk).unwrap().trim().parse().unwrap())
                        .collect()
                })
                .collect(),
        );
    }

    println!("\nPart one");
    let result = part_one(&numbers, &boards);
    println!("Answer: {}", result);

    println!("\nPart two");
    let result = part_two(&numbers, &boards);
    println!("Answer: {}", result);
}

fn part_one(numbers: &Vec<u8>, boards: &Vec<Board>) -> u32 {
    let mut board_states: Vec<Board> = (0..boards.len())
        .into_iter()
        .map(|_| new_board_state())
        .collect();

    for number in numbers {
        for board_idx in 0..boards.len() {
            update_board_state(&boards[board_idx], &mut board_states[board_idx], *number);

            if has_won(&board_states[board_idx]) {
                return get_score(&boards[board_idx], &board_states[board_idx], *number);
            }
        }
    }

    return 1;
}

fn part_two(numbers: &Vec<u8>, boards: &Vec<Board>) -> u32 {
    let mut board_states: Vec<Board> = (0..boards.len())
        .into_iter()
        .map(|_| new_board_state())
        .collect();
    let mut remaining_boards: Vec<u8> = (0..boards.len()).into_iter().map(|_| 1).collect();

    for number in numbers {
        for board_idx in 0..boards.len() {
            if remaining_boards[board_idx] == 1 {
                update_board_state(&boards[board_idx], &mut board_states[board_idx], *number);

                if has_won(&board_states[board_idx]) {
                    remaining_boards[board_idx] = 0;
                    if (&remaining_boards).into_iter().all(|state| *state == 0) {
                        return get_score(&boards[board_idx], &board_states[board_idx], *number);
                    }
                }
            }
        }
    }

    return 1;
}

fn new_board_state() -> Board {
    return (0..BOARD_HEIGTH)
        .into_iter()
        .map(|_| (0..BOARD_WIDTH).into_iter().map(|_| 0).collect())
        .collect();
}

fn update_board_state(board: &Board, board_state: &mut Board, number: u8) {
    for row in 0..BOARD_HEIGTH {
        for column in 0..BOARD_WIDTH {
            if board[row][column] == number {
                board_state[row][column] = 1;
            }
        }
    }
}

fn has_won(board_state: &Board) -> bool {
    // Look for horizontal wins
    for row in board_state {
        if row.into_iter().all(is_flagged) {
            return true;
        }
    }

    // Look for vertical wins
    for column in 0..BOARD_WIDTH {
        if (0..BOARD_HEIGTH)
            .into_iter()
            .map(|row| &board_state[row][column])
            .all(is_flagged)
        {
            return true;
        }
    }

    return false;
}

fn is_flagged(state: &u8) -> bool {
    return *state == 1;
}

fn get_score(board: &Board, board_state: &Board, number: u8) -> u32 {
    let mut sum: u32 = 0;
    for row in 0..BOARD_WIDTH {
        for column in 0..BOARD_HEIGTH {
            if board_state[row][column] == 0 {
                sum += board[row][column] as u32;
            }
        }
    }

    return sum * number as u32;
}
