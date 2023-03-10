use std::{collections::HashSet, ops::Add};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};
use pathfinding::prelude::dfs_reach;

use crate::utils::Closed;

pub fn solution(input: &str) -> String {
    let coords = parse(input);
    format!("{}, {}", part_one(&coords), part_two(&coords))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord3 {
    const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl Add for Coord3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

const UP: Coord3 = Coord3::new(0, 1, 0);
const DOWN: Coord3 = Coord3::new(0, -1, 0);
const LEFT: Coord3 = Coord3::new(-1, 0, 0);
const RIGHT: Coord3 = Coord3::new(1, 0, 0);
const FRONT: Coord3 = Coord3::new(0, 0, -1);
const BACK: Coord3 = Coord3::new(0, 0, 1);
const DIRS: [Coord3; 6] = [UP, DOWN, LEFT, RIGHT, FRONT, BACK];

fn parse(input: &str) -> Vec<Coord3> {
    let (_, coords) = all_consuming(p_coords)(input).expect("valid complete parse");
    coords
}

fn p_coords(input: &str) -> IResult<&str, Vec<Coord3>> {
    separated_list1(line_ending, p_coord)(input)
}

fn p_coord(input: &str) -> IResult<&str, Coord3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::i64(input)?;

    Ok((input, Coord3::new(x, y, z)))
}

fn part_one(coords: &[Coord3]) -> usize {
    let droplet: HashSet<_> = coords.iter().collect();

    droplet
        .iter()
        .map(|c| {
            DIRS.into_iter()
                .filter(|d| !droplet.contains(&(**c + *d)))
                .count()
        })
        .sum()
}

fn bounding_and_one(coords: &[Coord3]) -> (Closed, Closed, Closed) {
    let (min_x, max_x) = coords.iter().map(|c| c.x).minmax().into_option().unwrap();
    let (min_y, max_y) = coords.iter().map(|c| c.y).minmax().into_option().unwrap();
    let (min_z, max_z) = coords.iter().map(|c| c.z).minmax().into_option().unwrap();

    (
        Closed::new(min_x - 1, max_x + 1),
        Closed::new(min_y - 1, max_y + 1),
        Closed::new(min_z - 1, max_z + 1),
    )
}

fn part_two(coords: &[Coord3]) -> usize {
    let droplet: HashSet<_> = coords.iter().collect();
    let (x_range, y_range, z_range) = bounding_and_one(coords);

    let outside: HashSet<_> = dfs_reach(
        Coord3::new(x_range.start, y_range.start, z_range.start),
        |&c| {
            let droplet = &droplet;
            DIRS.into_iter().filter_map(move |d| {
                let n = d + c;
                if !droplet.contains(&n)
                    && x_range.contains(n.x)
                    && y_range.contains(n.y)
                    && z_range.contains(n.z)
                {
                    Some(n)
                } else {
                    None
                }
            })
        },
    )
    .collect();

    droplet
        .iter()
        .map(|c| {
            DIRS.into_iter()
                .filter(|d| {
                    let n = **c + *d;
                    outside.contains(&n)
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn example_part_one() {
        let coords = parse(INPUT);
        assert_eq!(part_one(&coords), 64);
    }

    #[test]
    fn example_part_two() {
        let coords = parse(INPUT);
        assert_eq!(part_two(&coords), 58);
    }
}
