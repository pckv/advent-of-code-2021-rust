use core::num::ParseIntError;
use core::str::FromStr;

#[derive(PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl FromStr for Position {
    type Err = ParseIntError;

    fn from_str(string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let parts: Vec<&str> = string.split(",").collect();
        return Ok(Position {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
        });
    }
}
