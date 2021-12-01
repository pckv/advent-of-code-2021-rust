use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn read_numbers_from_input(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
}
