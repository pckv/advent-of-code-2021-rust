use crate::common::read_numbers_from_input;

pub fn run() {
    let depth_measurements = read_numbers_from_input("src/day_01/input.txt");
    println!("Loaded {} depth measurements from input.txt", depth_measurements.len());

    // Count the number of times a depth measurement increases
    let mut increases = 0;
 
    // Add every increasing value
    for i in 1..depth_measurements.len() {
        if depth_measurements[i] > depth_measurements[i - 1] {
            increases += 1;
        }
    }

    println!("Answer: {}", increases);
}

