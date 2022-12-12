pub fn solution(input: &str) -> String {
    let elves = parse(input);
    format!("{}, {}", part_one(&elves), part_two(&elves))
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<u32>().expect("calories: unsigned integer"))
                .collect()
        })
        .collect()
}

fn part_one(elves: &[Vec<u32>]) -> u32 {
    elves
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .expect("non-empty list")
}

fn part_two(elves: &[Vec<u32>]) -> u32 {
    elves
        .iter()
        .fold(Top::<3>::new(), |mut top, elf| {
            top.update(elf.iter().sum());
            top
        })
        .sum()
}

struct Top<const N: usize>([u32; N]);

impl<const N: usize> Top<N> {
    fn new() -> Self {
        Top([0; N])
    }

    fn update(&mut self, mut n: u32) {
        use std::mem;
        for t in &mut self.0 {
            if *t < n {
                mem::swap(t, &mut n);
            }
        }
    }

    fn sum(&self) -> u32 {
        self.0.iter().sum()
    }
}
