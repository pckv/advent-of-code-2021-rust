use crate::common::get_file_lines;

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

fn part_one(lanternfish: &Vec<u32>) -> u32 {
    return simulate(&lanternfish, 80);
}

fn part_two(lanternfish: &Vec<u32>) -> u32 {
    return simulate(&lanternfish, 256);
}

fn advance(lanternfish: &mut Vec<u32>, day: u32) {
    let mut new_fish: u32 = 0;

    for fish in lanternfish.into_iter() {
        // Support waiting 9 days for new fish
        if day > FISH_TIMER && *fish == day {
            *fish = day % FISH_TIMER;
            new_fish += 1;
        } else if *fish == (day % FISH_TIMER) {
            new_fish += 1;
        }
    }

    for _ in 0..new_fish {
        lanternfish.push(day + NEW_FISH_TIMER);
    }
}

fn simulate(original_lanternfish: &Vec<u32>, days: u32) -> u32 {
    let mut lanternfish = original_lanternfish.clone();

    for day in 0..days {
        if day % 10 == 0 {
            println!("Simulation: day {}", day);
        }
        advance(&mut lanternfish, day);
    }

    return lanternfish.len() as u32;
}
