pub fn solution(input: &str) -> String {
    let guide = parse(input);
    format!("{}, {}", part_one(&guide), part_two(&guide))
}

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}
use Shape::*;

impl Shape {
    fn score(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    fn round(self, other: Self) -> Outcome {
        match (self, other) {
            (Rock, Scissor) => Win,
            (Paper, Rock) => Win,
            (Scissor, Paper) => Win,
            _ if self == other => Draw,
            _ => other.round(self).reverse(),
        }
    }

    fn should_choose(self, outcome: Outcome) -> Shape {
        [Rock, Paper, Scissor]
            .into_iter()
            .find(|s| s.round(self) == outcome)
            .expect("By definition")
    }
}

#[derive(Clone, Copy)]
enum Response {
    X,
    Y,
    Z,
}

impl Response {
    fn to_shape(self) -> Shape {
        match self {
            Response::X => Shape::Rock,
            Response::Y => Shape::Paper,
            Response::Z => Shape::Scissor,
        }
    }

    fn to_outcome(self) -> Outcome {
        match self {
            Response::X => Lose,
            Response::Y => Draw,
            Response::Z => Win,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}
use Outcome::*;

impl Outcome {
    fn reverse(self) -> Self {
        match self {
            Outcome::Win => Outcome::Lose,
            Outcome::Draw => Outcome::Draw,
            Outcome::Lose => Outcome::Win,
        }
    }

    fn score(self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

fn parse(input: &str) -> Vec<(Shape, Response)> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let shape = match bytes[0] {
                b'A' => Shape::Rock,
                b'B' => Shape::Paper,
                b'C' => Shape::Scissor,
                _ => unreachable!(),
            };
            let response = match bytes[2] {
                b'X' => Response::X,
                b'Y' => Response::Y,
                b'Z' => Response::Z,
                _ => unreachable!(),
            };

            (shape, response)
        })
        .collect()
}

fn part_one(guide: &[(Shape, Response)]) -> u32 {
    guide
        .iter()
        .map(|(s, r)| {
            let you = r.to_shape();
            you.score() + you.round(*s).score()
        })
        .sum()
}

fn part_two(guide: &[(Shape, Response)]) -> u32 {
    guide
        .iter()
        .map(|(s, r)| {
            let outcome = r.to_outcome();
            s.should_choose(outcome).score() + outcome.score()
        })
        .sum()
}
