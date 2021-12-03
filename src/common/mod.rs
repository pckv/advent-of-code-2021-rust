use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

pub fn get_file_reader(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).expect("file wasn't found.");
    return BufReader::new(file);
}

pub fn get_file_lines(file_path: &str) -> Lines<BufReader<File>> {
    return get_file_reader(file_path).lines();
}

pub fn read_numbers_from_input(file_path: &str) -> Vec<i32> {
    return get_file_reader(file_path)
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
}
