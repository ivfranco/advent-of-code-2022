use std::collections::HashMap;

pub fn solution(input: &str) -> String {
    format!("{}, {}", part_one(input), part_two(input))
}

fn no_repeat(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .enumerate()
        .all(|(i, b0)| bytes.iter().skip(i + 1).all(|b1| b1 != b0))
}

fn part_one(input: &str) -> usize {
    input
        .as_bytes()
        .windows(4)
        .position(no_repeat)
        .expect("valid input")
        + 4
}

fn part_two(input: &str) -> usize {
    const MESSAGE_MARKER: usize = 14;

    let bytes = input.as_bytes();
    let mut freq: HashMap<u8, u32> = HashMap::new();

    for &b in bytes.iter().take(MESSAGE_MARKER - 1) {
        *freq.entry(b).or_default() += 1;
    }

    for i in MESSAGE_MARKER - 1..bytes.len() {
        *freq.entry(bytes[i]).or_default() += 1;
        if freq.len() == MESSAGE_MARKER {
            return i + 1;
        }
        let rm = bytes[i + 1 - MESSAGE_MARKER];
        let entry = freq.entry(rm).or_default();
        *entry -= 1;
        if *entry == 0 {
            freq.remove(&rm);
        }
    }

    unreachable!("there must be a solution")
}
