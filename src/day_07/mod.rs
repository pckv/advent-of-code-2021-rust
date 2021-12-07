use crate::common::get_file_lines;
use core::cmp::min;

pub fn run() {
    let crab_positions: Vec<u32> = get_file_lines("src/day_07/input.txt")
        .map(|line| line.unwrap())
        .next()
        .unwrap()
        .split(",")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect();

    println!("Part one:");
    let result = part_one(&crab_positions);
    println!("Result: {}", result);

    println!("Part two:");
    let result = part_two(&crab_positions);
    println!("Result: {}", result);
}

fn part_one(crab_positions: &Vec<u32>) -> u32 {
    // Get median of all positions
    let median = get_median(&crab_positions);

    // Return the sum of abs(x - median)
    return crab_positions
        .into_iter()
        .fold(0, |acc, x| acc + get_abs_difference(*x, median));
}

fn part_two(crab_positions: &Vec<u32>) -> u32 {
    // Get the floored and ceiled average of all positions
    let average = get_average(&crab_positions);
    // Calculate fuel cost for each position
    let floored_cost = fuel_cost(&crab_positions, average.floor() as u32);
    let ceiled_cost = fuel_cost(&crab_positions, average.ceil() as u32);

    // Return the smallest value
    return min(floored_cost, ceiled_cost);
}

fn fuel_cost(positions: &Vec<u32>, num: u32) -> u32 {
    return positions.into_iter().fold(0, |acc, x| {
        acc + (1..get_abs_difference(*x, num) + 1)
            .into_iter()
            .sum::<u32>()
    });
}

fn get_abs_difference(a: u32, b: u32) -> u32 {
    return (a as i32 - b as i32).abs() as u32;
}

fn get_median(crab_positions: &Vec<u32>) -> u32 {
    let mut sorted = crab_positions.clone();
    sorted.sort_unstable();

    let middle = sorted.len() / 2;
    return sorted[middle];
}

fn get_average(crab_positions: &Vec<u32>) -> f32 {
    return crab_positions.into_iter().sum::<u32>() as f32 / crab_positions.len() as f32;
}
