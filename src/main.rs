use advent::load_or_download;
use advent_2022 as advent;
use std::env;
use std::error::Error;
use std::io::{self, ErrorKind};

type Solution = fn(&str) -> String;

fn main() -> Result<(), Box<dyn Error>> {
    let solutions: Vec<Solution> = vec![
        advent::day_1::solution,
        advent::day_2::solution,
        advent::day_3::solution,
        advent::day_4::solution,
        advent::day_5::solution,
        advent::day_6::solution,
        advent::day_7::solution,
        advent::day_8::solution,
        advent::day_9::solution,
        advent::day_10::solution,
        advent::day_11::solution,
        advent::day_12::solution,
        advent::day_13::solution,
        advent::day_14::solution,
        advent::day_15::solution,
        advent::day_16::solution,
        advent::day_17::solution,
        advent::day_18::solution,
        advent::day_19::solution,
        advent::day_20::solution,
        advent::day_21::solution,
        // advent::day_22::solution,
        // advent::day_23::solution,
        // advent::day_24::solution,
        // advent::day_25::solution,
    ];

    let mut args = env::args();
    args.next();
    let day = args
        .next()
        .and_then(|s| s.parse::<usize>().ok())
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "USAGE: EXEC DAY"))?;

    let input = load_or_download(day)?;
    let answer = solutions[day - 1](&input);

    println!("Answer to day {} is: {}", day, answer);

    Ok(())
}
