use crate::common::get_file_lines;
use std::collections::HashMap;

const FISH_TIMER: u32 = 7;
const NEW_FISH_TIMER: u32 = 9;

pub fn run() {
    let lanternfish: Vec<u32> = get_file_lines("src/day_06/input.txt")
        .map(|line| line.unwrap())
        .next()
        .unwrap()
        .split(",")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect();

    println!("Part one:");
    let result = part_one(&lanternfish);
    println!("Result: {}", result);

    println!("Part two:");
    let result = part_two(&lanternfish);
    println!("Result: {}", result);
}

fn part_one(lanternfish: &Vec<u32>) -> u64 {
    return simulate(&lanternfish, 80);
}

fn part_two(lanternfish: &Vec<u32>) -> u64 {
    return simulate(&lanternfish, 256);
}

fn advance(lanternfish: &mut HashMap<u32, u64>, day: u32) {
    let reproduce = day % FISH_TIMER;

    // Update todays reproducing fish with the new fish
    if day > FISH_TIMER && lanternfish.contains_key(&day) {
        *lanternfish.entry(reproduce).or_insert(0) += lanternfish[&day];
        lanternfish.remove(&day);
    }

    // Add new fish to future reproduction
    if lanternfish.contains_key(&reproduce) {
        lanternfish.insert(day + NEW_FISH_TIMER, lanternfish[&reproduce]);
    }
}

fn simulate(original_lanternfish: &Vec<u32>, days: u32) -> u64 {
    let mut lanternfish: HashMap<u32, u64> = HashMap::new();
    for fish in original_lanternfish {
        *lanternfish.entry(*fish).or_insert(0) += 1;
    }

    for day in 0..days {
        advance(&mut lanternfish, day);
    }

    return lanternfish.values().sum();
}
