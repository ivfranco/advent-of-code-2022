use std::{collections::HashSet, ops::Sub};

pub fn solution(input: &str) -> String {
    let motions = parse(input);
    format!("{}, {}", part_one(&motions), part_two(&motions))
}

#[derive(Clone, Copy)]
enum FourWay {
    U,
    L,
    D,
    R,
}

fn parse(input: &str) -> Vec<(FourWay, i32)> {
    let regex = Regex::new(r"(?P<dir>U|D|L|R) (?P<steps>\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = regex.captures(l).expect("valid input line");
            let dir = match &caps["dir"] {
                "U" => U,
                "L" => L,
                "D" => D,
                "R" => R,
                _ => unreachable!("valid direction"),
            };
            let steps = caps["steps"].parse().expect("valid steps");
            (dir, steps)
        })
        .collect()
}

use FourWay::*;

#[derive(Clone, Copy)]
enum EightWay {
    Four(FourWay),
    Eight(FourWay, FourWay),
}

use regex::Regex;
use EightWay::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn to_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn follow4(self, dir: FourWay) -> Self {
        let Pos { x, y } = self;

        let (nx, ny) = match dir {
            FourWay::U => (x, y + 1),
            FourWay::L => (x - 1, y),
            FourWay::D => (x, y - 1),
            FourWay::R => (x + 1, y),
        };

        Pos::new(nx, ny)
    }

    fn follow8(self, dir: EightWay) -> Self {
        match dir {
            EightWay::Four(f) => self.follow4(f),
            EightWay::Eight(f0, f1) => self.follow4(f0).follow4(f1),
        }
    }
}

fn tail_heading(head: Pos, tail: Pos) -> Option<EightWay> {
    match (head - tail).to_tuple() {
        (x, y) if x.abs() <= 1 && y.abs() <= 1 => None,
        (x, 0) if x > 0 => Some(Four(R)),
        (x, 0) if x < 0 => Some(Four(L)),
        (0, y) if y > 0 => Some(Four(U)),
        (0, y) if y < 0 => Some(Four(D)),
        (x, y) => {
            let v = if x > 0 { R } else { L };
            let h = if y > 0 { U } else { D };
            Some(Eight(v, h))
        }
    }
}

fn simulate_knots<const N: usize>(motions: &[(FourWay, i32)]) -> usize {
    let mut visited = HashSet::new();
    let mut knots = [Pos::default(); N];
    visited.insert(knots[N - 1]);

    for &(dir, steps) in motions {
        for _ in 0..steps {
            knots[0] = knots[0].follow4(dir);
            for i in 1..knots.len() {
                if let Some(d8) = tail_heading(knots[i - 1], knots[i]) {
                    knots[i] = knots[i].follow8(d8);
                }
            }
            visited.insert(knots[N - 1]);
        }
    }

    visited.len()
}

fn part_one(motions: &[(FourWay, i32)]) -> usize {
    simulate_knots::<2>(motions)
}

fn part_two(motions: &[(FourWay, i32)]) -> usize {
    simulate_knots::<10>(motions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const LONG_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn example_part_one() {
        let motions = parse(INPUT);
        assert_eq!(part_one(&motions), 13);
    }

    #[test]
    fn example_part_two() {
        let motions = parse(INPUT);
        assert_eq!(part_two(&motions), 1);
        let motions = parse(LONG_INPUT);
        assert_eq!(part_two(&motions), 36);
    }
}
