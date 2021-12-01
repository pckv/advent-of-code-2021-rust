use crate::common::read_numbers_from_input;

pub fn run() {
    let depth_measurements = read_numbers_from_input("src/day_01/input.txt");
    println!("Loaded {} depth measurements from input.txt", depth_measurements.len());

    println!("\nPart one");
    let result = part_one(&depth_measurements);
    println!("Answer: {}", result);

    println!("\nPart two");
    let result = part_two(&depth_measurements);
    println!("Answer: {}", result);
}

fn part_one(depth_measurements: &Vec<i32>) -> u32 {
    let mut increases: u32 = 0;
 
    // Add every increasing value
    for i in 1..depth_measurements.len() {
        if depth_measurements[i] > depth_measurements[i - 1] {
            increases += 1;
        }
    }

    return increases;
}

fn part_two(depth_measurements: &Vec<i32>) -> u32 {
    let mut increases: u32 = 0;
 
    for i in 3..depth_measurements.len() {
        let first_sum = sum_range(depth_measurements, i - 3, i - 1);
        let second_sum = sum_range(depth_measurements, i - 2, i);

        if second_sum > first_sum {
            increases += 1;
        }
    }

    return increases;
}

fn sum_range(numbers: &Vec<i32>, start_index: usize, end_index: usize) -> i32 {
    let mut sum = 0;
    for i in start_index..end_index + 1 {
        sum += numbers[i];
    }

    return sum;
}
