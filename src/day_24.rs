use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::line_ending,
    combinator::all_consuming,
    multi::many1,
    sequence::{delimited, tuple},
    IResult, Parser,
};
use pathfinding::prelude::astar;

use crate::utils::{Coord, DOWN, LEFT, RIGHT, STAY, UP};

pub fn solution(input: &str) -> String {
    let valley = parse(input);
    format!("{}, {}", part_one(&valley), part_two(&valley))
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}
use Dir::*;

impl Dir {
    fn to_coord(self) -> Coord {
        match self {
            Up => UP,
            Left => LEFT,
            Down => DOWN,
            Right => RIGHT,
        }
    }
}

#[derive(Clone, Copy)]
enum Ground {
    Blizzard(Dir),
    Clear,
}
use Ground::*;

struct Valley {
    width: usize,
    height: usize,
    grounds: Vec<Vec<Ground>>,
}

fn parse(input: &str) -> Valley {
    let (_, valley) = all_consuming(p_valley)(input).expect("valid complete parse");
    valley
}

fn p_valley(input: &str) -> IResult<&str, Valley> {
    let (input, _) = delimited(tag("#."), take_while1(|c: char| c == '#'), line_ending)(input)?;
    let (input, grounds) = many1(p_valley_line)(input)?;
    let (input, _) = tuple((take_while1(|c: char| c == '#'), tag(".#")))(input)?;

    let width = grounds[0].len();
    let height = grounds.len();

    let valley = Valley {
        width,
        height,
        grounds,
    };
    Ok((input, valley))
}

fn p_valley_line(input: &str) -> IResult<&str, Vec<Ground>> {
    delimited(tag("#"), many1(p_blizzard), tuple((tag("#"), line_ending)))(input)
}

fn p_blizzard(input: &str) -> IResult<&str, Ground> {
    alt((
        tag("^").map(|_| Blizzard(Up)),
        tag("<").map(|_| Blizzard(Left)),
        tag("v").map(|_| Blizzard(Down)),
        tag(">").map(|_| Blizzard(Right)),
        tag(".").map(|_| Clear),
    ))(input)
}

fn lcm(a: usize, b: usize) -> usize {
    let (a, b) = if a < b { (b, a) } else { (a, b) };

    let mut m = a;
    while m % b != 0 {
        m += a;
    }
    m
}

fn update_passing(valley: &Valley, mut coord: Coord, d: Coord, passing: &mut [Vec<Vec<bool>>]) {
    let repeat = lcm(valley.width, valley.height);

    for i in 0..repeat {
        passing[coord.y as usize][coord.x as usize][i] = true;

        coord = coord + d;
        if coord.y >= valley.height as i64 {
            coord.y = 0
        } else if coord.y < 0 {
            coord.y = (valley.height - 1) as i64
        } else if coord.x >= valley.width as i64 {
            coord.x = 0
        } else if coord.x < 0 {
            coord.x = (valley.width - 1) as i64
        }
    }
}

struct Map {
    repeat: usize,
    passing: Vec<Vec<Vec<bool>>>,
}

impl Map {
    fn new(valley: &Valley) -> Self {
        let repeat = lcm(valley.width, valley.height);
        let mut passing = vec![vec![vec![false; repeat]; valley.width]; valley.height];

        for (y, row) in valley.grounds.iter().enumerate() {
            for (x, &g) in row.iter().enumerate() {
                if let Blizzard(d) = g {
                    update_passing(
                        valley,
                        Coord::new(x as i64, y as i64),
                        d.to_coord(),
                        &mut passing,
                    );
                }
            }
        }

        Self { repeat, passing }
    }

    fn width(&self) -> i64 {
        self.passing[0].len() as i64
    }

    fn height(&self) -> i64 {
        self.passing.len() as i64
    }

    fn start(&self) -> Coord {
        Coord::new(0, -1)
    }

    fn goal(&self) -> Coord {
        Coord::new(self.width() - 1, self.height())
    }

    fn in_valley(&self, coord: Coord) -> bool {
        coord.x >= 0 && coord.x < self.width() && coord.y >= 0 && coord.y < self.height()
    }

    fn passible(&self, coord: Coord, epoch: usize) -> bool {
        coord == self.start()
            || coord == self.goal()
            || (self.in_valley(coord)
                && !self.passing[coord.y as usize][coord.x as usize][epoch % self.repeat])
    }

    fn moves(&self, actor: Expedition) -> Vec<(Expedition, i64)> {
        [UP, DOWN, LEFT, RIGHT, STAY]
            .into_iter()
            .map(|d| actor.coord + d)
            .filter(|next| self.passible(*next, actor.epoch + 1))
            .map(|next| {
                (
                    Expedition {
                        coord: next,
                        epoch: actor.epoch + 1,
                    },
                    1,
                )
            })
            .collect()
    }

    fn triple_moves(&self, actor: TripleExpedition) -> Vec<(TripleExpedition, i64)> {
        self.moves(Expedition {
            coord: actor.coord,
            epoch: actor.epoch,
        })
        .into_iter()
        .map(|(next, cost)| {
            let stage = if actor.stage == Init && next.coord == self.goal() {
                FirstGoal
            } else if actor.stage == FirstGoal && next.coord == self.start() {
                SecondStart
            } else {
                actor.stage
            };

            let triple = TripleExpedition {
                coord: next.coord,
                epoch: next.epoch,
                stage,
            };

            (triple, cost)
        })
        .collect()
    }

    fn heuristic(&self, coord: Coord) -> i64 {
        coord.manhattan_distance(self.goal())
    }

    fn triple_heuristic(&self, actor: TripleExpedition) -> i64 {
        match actor.stage {
            ExpeditionStage::Init => {
                actor.coord.manhattan_distance(self.goal())
                    + self.goal().manhattan_distance(self.start()) * 2
            }
            ExpeditionStage::FirstGoal => {
                actor.coord.manhattan_distance(self.start())
                    + self.start().manhattan_distance(self.goal())
            }
            ExpeditionStage::SecondStart => actor.coord.manhattan_distance(self.goal()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Expedition {
    coord: Coord,
    epoch: usize,
}

fn part_one(valley: &Valley) -> i64 {
    let map = Map::new(valley);
    let (_path, shortest) = astar(
        &Expedition {
            coord: map.start(),
            epoch: 0,
        },
        |state| map.moves(*state),
        |state| map.heuristic(state.coord),
        |state| state.coord == map.goal(),
    )
    .expect("shortest path");

    shortest
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ExpeditionStage {
    Init,
    FirstGoal,
    SecondStart,
}
use ExpeditionStage::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct TripleExpedition {
    coord: Coord,
    epoch: usize,
    stage: ExpeditionStage,
}

fn part_two(valley: &Valley) -> i64 {
    let map = Map::new(valley);
    let (_path, shortest) = astar(
        &TripleExpedition {
            coord: map.start(),
            epoch: 0,
            stage: Init,
        },
        |state| map.triple_moves(*state),
        |state| map.triple_heuristic(*state),
        |state| state.stage == SecondStart && state.coord == map.goal(),
    )
    .expect("shortest path");

    shortest
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn example_part_one() {
        let valley = parse(INPUT);
        assert_eq!(part_one(&valley), 18);
    }

    #[test]
    fn example_part_two() {
        let valley = parse(INPUT);
        assert_eq!(part_two(&valley), 54);
    }
}
