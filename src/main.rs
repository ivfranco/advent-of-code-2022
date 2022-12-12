use advent_2022::load_or_download;
use std::env;
use std::error::Error;
use std::io::{self, ErrorKind};

type Solution = fn(&str) -> String;

fn main() -> Result<(), Box<dyn Error>> {
    let solutions: Vec<Solution> = vec![
        advent_2022::day_1::solution,
        advent_2022::day_2::solution,
        advent_2022::day_3::solution,
        advent_2022::day_4::solution,
        advent_2022::day_5::solution,
        advent_2022::day_6::solution,
        advent_2022::day_7::solution,
        advent_2022::day_8::solution,
        advent_2022::day_9::solution,
        advent_2022::day_10::solution,
        advent_2022::day_11::solution,
        advent_2022::day_12::solution,
        // advent_2022::day_13::solution,
        // advent_2022::day_14::solution,
        // advent_2022::day_15::solution,
        // advent_2022::day_16::solution,
        // advent_2022::day_17::solution,
        // advent_2022::day_18::solution,
        // advent_2022::day_19::solution,
        // advent_2022::day_20::solution,
        // advent_2022::day_21::solution,
        // advent_2022::day_22::solution,
        // advent_2022::day_23::solution,
        // advent_2022::day_24::solution,
        // advent_2022::day_25::solution,
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
