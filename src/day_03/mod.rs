use crate::common::get_file_lines;

fn line_to_bytes(line: &String) -> Vec<u8> {
    return line
        .chars()
        .map(|bit| match bit {
            '0' => 0,
            _ => 1,
        })
        .collect();
}

pub fn run() {
    let bytes: Vec<Vec<u8>> = get_file_lines("src/day_03/input.txt")
        .map(|line| line_to_bytes(&line.unwrap()))
        .collect();
    println!("\nPart one");
    let result = part_one(&bytes);
    println!("Answer: {}", result);

    println!("\nPart two");
    let result = part_two(&bytes);
    println!("Answer: {}", result);
}

fn filter_bits(bytes: &Vec<Vec<u8>>, filter: fn(u32, u32) -> u8) -> Vec<u8> {
    let positions = bytes[0].len();

    return (0..positions)
        .map(|position| filter_bit(bytes, filter, position))
        .collect();
}

fn filter_bit(bytes: &Vec<Vec<u8>>, filter: fn(u32, u32) -> u8, position: usize) -> u8 {
    let mut zeros: u32 = 0;
    let mut ones: u32 = 0;
    for byte in bytes {
        match byte[position] {
            0 => zeros += 1,
            _ => ones += 1,
        }
    }

    return filter(zeros, ones);
}

fn bits_to_u32(bits: &Vec<u8>) -> u32 {
    return bits
        .into_iter()
        .fold(0, |acc, &bit| (acc << 1) | bit as u32);
}

fn filter_common_bits(zeros: u32, ones: u32) -> u8 {
    return if zeros > ones { 0 } else { 1 };
}

fn filter_uncommon_bits(zeros: u32, ones: u32) -> u8 {
    return if ones >= zeros { 0 } else { 1 };
}

fn part_one(bytes: &Vec<Vec<u8>>) -> u32 {
    let gamma_bits = filter_bits(&bytes, filter_common_bits);
    let gamma = bits_to_u32(&gamma_bits);

    let epsilon_bits = filter_bits(&bytes, filter_uncommon_bits);
    let epsilon = bits_to_u32(&epsilon_bits);

    return gamma * epsilon;
}

fn get_rating(bytes: &Vec<Vec<u8>>, filter: fn(u32, u32) -> u8) -> u32 {
    let mut remaining_bytes = bytes.to_vec();
    let positions = bytes[0].len();

    for position in 0..positions {
        if remaining_bytes.len() == 1 {
            break;
        }

        let bit = filter_bit(&remaining_bytes, filter, position);
        remaining_bytes = remaining_bytes
            .into_iter()
            .filter(|byte| byte[position] == bit)
            .collect();
    }

    return bits_to_u32(&remaining_bytes[0]);
}

fn part_two(bytes: &Vec<Vec<u8>>) -> u32 {
    let oxygen_rating = get_rating(&bytes, filter_common_bits);
    let co2_scrubber_rating = get_rating(&bytes, filter_uncommon_bits);

    return oxygen_rating * co2_scrubber_rating;
}
