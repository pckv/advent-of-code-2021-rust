#[macro_use]
extern crate clap;
use clap::App;

extern crate chrono;
use chrono::Local;

mod day_01;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let day_argument = matches.value_of("DAY").unwrap_or_default();
    let day = get_day(day_argument);

    println!("Running task for day {}...", day);
    match day {
        1 => day_01::run(),
        _ => println!("No task for day {}", day),
    }
}

fn get_day(day: &str) -> u8 {
    if day == "" {
        println!("No day specified, using current day");
        let now = Local::now();
        let day = now.format("%d").to_string();
        return day.parse::<u8>().unwrap();
    }

    return day.parse::<u8>().unwrap();
}
