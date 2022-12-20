use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use pathfinding::prelude::astar;

pub fn solution(input: &str) -> String {
    let blueprints = parse(input);
    format!("{}, {}", part_one(&blueprints), part_two(&blueprints))
}

#[derive(Debug, Default, Clone, Copy)]
struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

#[derive(Debug, Default, Clone, Copy)]
struct Blueprint {
    id: i32,
    ore_cost: Cost,
    clay_cost: Cost,
    obsidian_cost: Cost,
    geode_cost: Cost,
}

fn parse(input: &str) -> Vec<Blueprint> {
    let (_, blueprints) = all_consuming(separated_list1(line_ending, p_blueprint))(input)
        .expect("valid complete parse");
    blueprints
}

fn p_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), i32, tag(": "))(input)?;
    let (input, ore_cost) = delimited(tag("Each ore robot costs "), i32, tag(" ore. "))
        .map(|ore| Cost {
            ore,
            ..Default::default()
        })
        .parse(input)?;
    let (input, clay_cost) = delimited(tag("Each clay robot costs "), i32, tag(" ore. "))
        .map(|ore| Cost {
            ore,
            ..Default::default()
        })
        .parse(input)?;
    let (input, obsidian_cost) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(i32, tag(" ore and "), i32),
        tag(" clay. "),
    )
    .map(|(ore, clay)| Cost {
        ore,
        clay,
        ..Default::default()
    })
    .parse(input)?;
    let (input, geode_cost) = delimited(
        tag("Each geode robot costs "),
        separated_pair(i32, tag(" ore and "), i32),
        tag(" obsidian."),
    )
    .map(|(ore, obsidian)| Cost {
        ore,
        obsidian,
        ..Default::default()
    })
    .parse(input)?;

    Ok((
        input,
        Blueprint {
            id,
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
        },
    ))
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    remaining: i32,
    /// [ore, clay, obsidian, geode]
    robots: [i32; 4],
    resources: [i32; 4],
}

impl State {
    fn new(remaining: i32) -> Self {
        Self {
            remaining,
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
        }
    }

    fn afford(&self, cost: Cost) -> Option<Self> {
        if self.resources[ORE] >= cost.ore
            && self.resources[CLAY] >= cost.clay
            && self.resources[OBSIDIAN] >= cost.obsidian
        {
            let mut next = *self;
            next.resources[ORE] -= cost.ore;
            next.resources[CLAY] -= cost.clay;
            next.resources[OBSIDIAN] -= cost.obsidian;
            Some(next)
        } else {
            None
        }
    }

    fn moves(&self, blueprint: &Blueprint) -> Vec<(Self, i32)> {
        let prev_robots = self.robots;
        let mut nexts = vec![*self];

        if let Some(mut next) = self.afford(blueprint.ore_cost) {
            next.robots[ORE] += 1;
            nexts.push(next);
        }
        if let Some(mut next) = self.afford(blueprint.clay_cost) {
            next.robots[CLAY] += 1;
            nexts.push(next);
        }
        if let Some(mut next) = self.afford(blueprint.obsidian_cost) {
            next.robots[OBSIDIAN] += 1;
            nexts.push(next);
        }
        if let Some(mut next) = self.afford(blueprint.geode_cost) {
            next.robots[GEODE] += 1;
            nexts.push(next);
        }

        nexts
            .into_iter()
            .map(move |mut s| {
                s.remaining -= 1;
                #[allow(clippy::needless_range_loop)]
                for i in 0..4 {
                    s.resources[i] += prev_robots[i];
                }
                (s, -prev_robots[GEODE])
            })
            .collect()
    }

    fn heuristic(&self, blueprint: &Blueprint) -> i32 {
        // assume an ore robot and an obsidian robot can be built for free each minute, and a geode
        // robot can be built in the same minute if affordable
        let mut free = *self;

        for _ in 0..self.remaining {
            let prev_robots = free.robots;
            if let Some(next) = free.afford(blueprint.geode_cost) {
                free = next;
                free.robots[GEODE] += 1;
            }
            free.robots[ORE] += 1;
            free.robots[OBSIDIAN] += 1;

            #[allow(clippy::needless_range_loop)]
            for i in 0..4 {
                free.resources[i] += prev_robots[i];
            }
        }

        self.resources[GEODE] - free.resources[GEODE]
    }

    fn done(&self) -> bool {
        debug_assert!(self.remaining >= 0);
        self.remaining == 0
    }
}

fn part_one(blueprints: &[Blueprint]) -> i32 {
    blueprints
        .iter()
        .map(|b| {
            let (_, cost) = astar(
                &State::new(24),
                |state| state.moves(b),
                |state| state.heuristic(b),
                |state| state.done(),
            )
            .unwrap();

            b.id * -cost
        })
        .sum()
}

fn part_two(blueprints: &[Blueprint]) -> i32 {
    blueprints
        .iter()
        .take(3)
        .map(|b| {
            let (_, cost) = astar(
                &State::new(32),
                |state| state.moves(b),
                |state| state.heuristic(b),
                |state| state.done(),
            )
            .unwrap();
            -cost
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn example_part_one() {
        let blueprints = parse(INPUT);
        assert_eq!(part_one(&blueprints), 33);
    }

    #[test]
    fn example_part_two() {
        let blueprints = parse(INPUT);
        assert_eq!(part_two(&blueprints), 56 * 62);
    }
}
