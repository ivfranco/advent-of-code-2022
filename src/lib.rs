pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod utils;
// pub mod day_16;
// pub mod day_17;
// pub mod day_18;
// pub mod day_19;
// pub mod day_20;
// pub mod day_21;
// pub mod day_22;
// pub mod day_23;
// pub mod day_24;
// pub mod day_25;

use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

const SESSION_KEY: &str = include_str!("../SESSION_KEY");
const ADVENT_URL: &str = "https://adventofcode.com/2022";

pub fn load_or_download(id: usize) -> Result<String, Box<dyn Error>> {
    let input_file_path = format!("./inputs/day_{}", id);

    if !Path::new(&input_file_path).exists() {
        println!("Downloading input for day {}...", id);

        let url = format!("{}/day/{}/input", ADVENT_URL, id);
        let cookie = format!("session={}", SESSION_KEY);
        let mut input = ureq::get(&url)
            .set("Cookie", &cookie)
            .call()
            .map_err(|e| {
                eprintln!("Session key expired or invalid?");
                e
            })?
            .into_string()?
            .into_bytes();

        // for some reason inputs downloaded this way may contain an additional \n at the end
        if input.last() == Some(&b'\n') {
            input.truncate(input.len() - 1);
        }

        if input.starts_with(b"Puzzle inputs differ by user") {
            return Err("Session key expired or invalid?".into());
        }

        fs::write(&input_file_path, &input)?;
        println!("Downloaded input for day {}", id);
    }

    let mut input = String::new();
    let mut file = File::open(&input_file_path)?;
    file.read_to_string(&mut input)?;

    Ok(input)
}
