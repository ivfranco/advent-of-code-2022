use regex::Regex;

pub fn solution(input: &str) -> String {
    let (crates, steps) = parse(input);
    format!(
        "{}, {}",
        part_one(&crates, &steps),
        part_two(&crates, &steps)
    )
}

#[derive(Clone)]
struct Supplies(Vec<Vec<u8>>);

#[derive(Clone, Copy)]
struct Step {
    repeat: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Supplies, Vec<Step>) {
    let line_len = input.find('\n').expect("valid input");
    let supplies_len = (line_len + 1) / 4;
    let mut supplies: Vec<Vec<u8>> = vec![vec![]; supplies_len];

    let mut lines = input.lines();

    for l in (&mut lines).take_while(|l| l.starts_with('[')) {
        let b = l.as_bytes();
        for (supply_idx, line_idx) in (1usize..line_len).step_by(4).enumerate() {
            let c = b[line_idx];
            if c.is_ascii_uppercase() {
                supplies[supply_idx].push(c);
            }
        }
    }

    for c in supplies.iter_mut() {
        c.reverse();
    }

    assert_eq!(lines.next(), Some(""));

    let step_pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let steps = lines
        .map(|l| {
            let caps = step_pattern.captures(l).expect("valid input");
            let repeat = caps[1].parse().unwrap();
            let from = caps[2].parse::<usize>().unwrap() - 1;
            let to = caps[3].parse::<usize>().unwrap() - 1;

            assert_ne!(from, to);

            Step { repeat, from, to }
        })
        .collect();

    (Supplies(supplies), steps)
}

fn part_one(supplies: &Supplies, steps: &[Step]) -> String {
    let mut supplies = supplies.0.clone();

    for &Step { repeat, from, to } in steps {
        for _ in 0..repeat {
            if let Some(c) = supplies[from].pop() {
                supplies[to].push(c);
            }
        }
    }

    supplies
        .iter()
        .map(|s| {
            let c = *s.last().expect("at least one crate each supply");
            char::from(c)
        })
        .collect()
}

fn part_two(supplies: &Supplies, steps: &[Step]) -> String {
    let mut supplies = supplies.0.clone();

    for &Step { repeat, from, to } in steps {
        let from_len = supplies[from].len();
        let cut_start = from_len - from_len.min(repeat);
        let cut = supplies[from].split_off(cut_start);
        supplies[to].extend_from_slice(&cut);
    }

    supplies
        .iter()
        .map(|s| {
            let c = *s.last().expect("at least one crate each supply");
            char::from(c)
        })
        .collect()
}
