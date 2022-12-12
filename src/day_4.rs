pub fn solution(input: &str) -> String {
    let assignments = parse(input);
    format!("{}, {}", part_one(&assignments), part_two(&assignments))
}

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn new(min: u32, max: u32) -> Self {
        assert!(min <= max);
        Self { min, max }
    }

    fn covers(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.max >= other.min && self.min <= other.max
    }
}

fn parse(input: &str) -> Vec<(Range, Range)> {
    let regex = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input
        .lines()
        .map(|l| {
            let cap = regex.captures(l).unwrap();
            let range_0 = Range::new(cap[1].parse().unwrap(), cap[2].parse().unwrap());
            let range_1 = Range::new(cap[3].parse().unwrap(), cap[4].parse().unwrap());

            (range_0, range_1)
        })
        .collect()
}

fn part_one(assignments: &[(Range, Range)]) -> usize {
    assignments
        .iter()
        .filter(|(r0, r1)| r0.covers(r1) || r1.covers(r0))
        .count()
}

fn part_two(assignments: &[(Range, Range)]) -> usize {
    assignments
        .iter()
        .filter(|(r0, r1)| r0.overlaps(r1))
        .count()
}
