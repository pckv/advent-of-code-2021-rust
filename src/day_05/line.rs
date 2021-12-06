use crate::day_05::position::Position;
use core::num::ParseIntError;
use core::str::FromStr;

pub struct Line {
    pub start: Position,
    pub end: Position,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let parts: Vec<&str> = string.split(" -> ").collect();

        return Ok(Line {
            start: parts[0].parse()?,
            end: parts[1].parse()?,
        });
    }
}
