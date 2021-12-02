#[macro_use]
extern crate clap;
use clap::App;

extern crate chrono;
use chrono::Local;

mod common;
mod day_01;
mod day_02;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let day_argument = matches.value_of("DAY").unwrap_or_default();
    let day = if day_argument == "" {
        println!("No day specified, using current day");
        get_today()
    } else {
        day_argument.parse::<u8>().unwrap()
    };

    println!("Running task for day {}...", day);
    run_day(day);
}

fn run_day(day: u8) {
    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        _ => println!("No task for day {}", day),
    }
}

fn get_today() -> u8 {
    let now = Local::now();
    let day = now.format("%d").to_string();
    return day.parse::<u8>().unwrap();
}
