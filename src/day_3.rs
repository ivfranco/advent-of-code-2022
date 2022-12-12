use std::collections::{HashMap, HashSet};

pub fn solution(input: &str) -> String {
    let rucksacks = parse(input);
    format!("{}, {}", part_one(&rucksacks), part_two(&rucksacks))
}

fn priority(item: u8) -> u32 {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as u32,
        b'A'..=b'Z' => (item - b'A' + 27) as u32,
        _ => unreachable!("invalid item"),
    }
}

struct Rucksack<'a> {
    fst: &'a [u8],
    snd: &'a [u8],
}

fn parse(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|l| {
            let b = l.as_bytes();
            let (fst, snd) = b.split_at(b.len() / 2);
            Rucksack { fst, snd }
        })
        .collect()
}

fn part_one(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(|sack| {
            let set: HashSet<u8> = sack.fst.iter().copied().collect();
            let item = sack
                .snd
                .iter()
                .find(|item| set.contains(item))
                .expect("valid input");

            priority(*item)
        })
        .sum()
}

fn part_two(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .chunks_exact(3)
        .map(|sacks| {
            let mut bitflags = HashMap::<u8, u8>::new();
            for (i, sack) in sacks.iter().enumerate() {
                for item in sack.fst.iter().chain(sack.snd) {
                    *bitflags.entry(*item).or_default() |= 0b1 << i;
                }
            }

            let badge = bitflags
                .iter()
                .find_map(
                    |(item, flag)| {
                        if *flag == 0b111 {
                            Some(*item)
                        } else {
                            None
                        }
                    },
                )
                .expect("valid input");

            priority(badge)
        })
        .sum()
}
