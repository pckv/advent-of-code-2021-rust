use crate::common::get_file_lines;
use core::ops::Range;

// A note on terminology: a "byte" in this module is of variable length
// and as such may be any number of bits long. The provided input uses
// a bit-length of 12
type Bit = u8;
type Byte = Vec<Bit>;

type FilterFn = fn(u32, u32) -> Bit;

pub fn run() {
    // Convert bytes to a 2D-array of binary digits
    let bytes: Vec<Byte> = get_file_lines("src/day_03/input.txt")
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|bit| match bit {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("invalid bit"),
                })
                .collect()
        })
        .collect();

    println!("\nPart one");
    let result = part_one(&bytes);
    println!("Answer: {}", result);

    println!("\nPart two");
    let result = part_two(&bytes);
    println!("Answer: {}", result);
}

fn part_one(bytes: &Vec<Byte>) -> u32 {
    let gamma = byte_to_u32(&filter_bits(&bytes, filter_common_bits));
    let epsilon = byte_to_u32(&filter_bits(&bytes, filter_uncommon_bits));

    return gamma * epsilon;
}

fn part_two(bytes: &Vec<Byte>) -> u32 {
    let oxygen_rating = get_rating(&bytes, filter_common_bits);
    let co2_scrubber_rating = get_rating(&bytes, filter_uncommon_bits);

    return oxygen_rating * co2_scrubber_rating;
}

fn iter_positions(bytes: &Vec<Byte>) -> Range<usize> {
    return 0..bytes[0].len();
}

fn filter_bits(bytes: &Vec<Byte>, filter: FilterFn) -> Byte {
    return iter_positions(&bytes)
        .map(|position| filter_bit(bytes, filter, position))
        .collect();
}

fn filter_bit(bytes: &Vec<Byte>, filter: FilterFn, position: usize) -> Bit {
    let (zeros, ones) =
        bytes
            .into_iter()
            .fold((0, 0), |(zeros, ones), byte| match byte[position] {
                0 => (zeros + 1, ones),
                _ => (zeros, ones + 1),
            });

    return filter(zeros, ones);
}

fn byte_to_u32(bits: &Byte) -> u32 {
    return bits
        .into_iter()
        .fold(0, |acc, &bit| (acc << 1) | bit as u32);
}

fn filter_common_bits(zeros: u32, ones: u32) -> Bit {
    return if zeros > ones { 0 } else { 1 };
}

fn filter_uncommon_bits(zeros: u32, ones: u32) -> Bit {
    return if ones >= zeros { 0 } else { 1 };
}

fn get_rating(bytes: &Vec<Byte>, filter: FilterFn) -> u32 {
    let mut remaining_bytes = bytes.to_vec();

    for position in iter_positions(&bytes) {
        if remaining_bytes.len() == 1 {
            break;
        }

        let bit = filter_bit(&remaining_bytes, filter, position);
        remaining_bytes = remaining_bytes
            .into_iter()
            .filter(|byte| byte[position] == bit)
            .collect();
    }

    return byte_to_u32(&remaining_bytes[0]);
}
