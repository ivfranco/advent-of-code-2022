use std::collections::{hash_map::Entry, BTreeSet, HashMap};
use std::hash::Hash;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{i64, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use pathfinding::prelude::{astar, dijkstra_all};

pub fn solution(input: &str) -> String {
    let valves = parse(input);
    let compressed = compress(&valves);
    let (ids, start) = identifiers(&compressed);
    format!("{}, {}", part_one(&compressed), part_two(&ids, start))
}

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow: i64,
    exit: Vec<&'a str>,
}

fn parse(input: &str) -> Vec<Valve> {
    let (_, valves) = all_consuming(p_valves)(input).expect("valid complete parse");
    valves
}

#[derive(Debug)]
struct CompressedValve<'a> {
    name: &'a str,
    flow: i64,
    exit: Vec<(&'a str, i64)>,
}

struct IdCache<T> {
    cache: HashMap<T, usize>,
}

impl<T: PartialEq + Eq + Hash> IdCache<T> {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn id(&mut self, item: T) -> usize {
        let len = self.cache.len() as usize;
        match self.cache.entry(item) {
            Entry::Vacant(e) => {
                e.insert(len);
                len
            }
            Entry::Occupied(e) => *e.get(),
        }
    }
}

fn compress<'a>(valves: &'a [Valve]) -> HashMap<&'a str, CompressedValve<'a>> {
    let valves: HashMap<&str, &Valve> = valves.iter().map(|v| (v.name, v)).collect();
    let mut compressed = HashMap::new();

    for &k in valves.keys().filter(|&&k| k == START || valves[k].flow > 0) {
        let predecessors = dijkstra_all(&k, |n| {
            if *n != k && valves[n].flow > 0 {
                return vec![];
            }

            valves[n].exit.iter().map(|e| (*e, 1)).collect()
        });

        let exit = predecessors
            .iter()
            .filter(|(n, _)| valves[**n].flow > 0)
            .map(|(n, (_, c))| (*n, *c))
            .collect();

        compressed.insert(
            k,
            CompressedValve {
                name: k,
                flow: valves[k].flow,
                exit,
            },
        );
    }

    compressed
}

struct IdValve {
    name: usize,
    flow: i64,
    exit: Vec<(usize, i64)>,
}

fn identifiers(valves: &HashMap<&str, CompressedValve>) -> (HashMap<usize, IdValve>, usize) {
    let mut cache = IdCache::new();

    for &name in valves.keys() {
        cache.id(name);
    }

    let id_valves = valves
        .values()
        .map(|v| {
            (
                cache.id(v.name),
                IdValve {
                    name: cache.id(v.name),
                    flow: v.flow,
                    exit: v.exit.iter().map(|(n, c)| (cache.id(n), *c)).collect(),
                },
            )
        })
        .collect();

    let start = cache.id(START);

    (id_valves, start)
}

fn p_valves(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(line_ending, p_valve)(input)
}

fn p_valve(input: &str) -> IResult<&str, Valve> {
    let (input, name) = preceded(tag("Valve "), take(2usize))(input)?;
    let (input, flow) = preceded(tag(" has flow rate="), i64)(input)?;
    let (input, exit) = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), take(2usize)),
    )(input)?;

    Ok((input, Valve { name, flow, exit }))
}

const START: &str = "AA";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State<'a> {
    remaining: i64,
    current: &'a str,
    opened: BTreeSet<&'a str>,
}

impl<'a> State<'a> {
    fn new() -> Self {
        Self {
            remaining: 30,
            current: START,
            opened: BTreeSet::new(),
        }
    }

    fn opened(&self, valve: &str) -> bool {
        self.opened.contains(valve)
    }

    fn move_to(&self, valve: &'a str, cost: i64) -> Option<(Self, i64)> {
        if self.remaining >= cost {
            let mut next = self.clone();

            next.remaining -= cost;
            next.current = valve;

            Some((next, 0))
        } else {
            None
        }
    }

    fn open(&self, valves: &HashMap<&str, CompressedValve>) -> (Self, i64) {
        if self.opened.contains(self.current) {
            panic!("check opened");
        }

        let mut next = self.clone();
        next.opened.insert(next.current);
        next.remaining -= 1;
        let cost = next.remaining * valves[next.current].flow;

        (next, -cost)
    }

    fn finished(&self, valves: &HashMap<&str, CompressedValve>) -> bool {
        assert!(self.remaining >= 0);
        self.remaining == 0 || self.opened.len() == valves.len()
    }

    fn heuristic(&self, sorted_by_flow: &[(&str, i64)]) -> i64 {
        let mut h = 0;
        let mut r = self.remaining;
        let mut i = 0;

        while r >= 1 && i < sorted_by_flow.len() {
            let inc = sorted_by_flow[i..]
                .iter()
                .position(|(name, _)| !self.opened(name));

            if let Some(inc) = inc {
                i += inc;
            } else {
                break;
            }

            r -= 1;
            h += sorted_by_flow[i].1 * r;
            i += 1;
            r -= 1;
        }

        -h
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct BitSet(u64);

impl BitSet {
    fn new() -> Self {
        Self(0)
    }

    fn insert(&mut self, k: usize) {
        self.0 |= 0b1 << k;
    }

    fn contains(&self, k: usize) -> bool {
        self.0 & (0b1 << k) != 0
    }

    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct ElephantState {
    remaining: i64,
    you: (usize, i64),
    elephant: (usize, i64),
    opened: BitSet,
}

impl ElephantState {
    fn new(start: usize) -> Self {
        ElephantState {
            remaining: 26,
            you: (start, 0),
            elephant: (start, 0),
            opened: BitSet::new(),
        }
    }

    fn opened(&self, valve: usize) -> bool {
        self.opened.contains(valve)
    }

    fn finished(&self, valves: &HashMap<usize, IdValve>) -> bool {
        assert!(self.remaining >= 0);
        self.remaining == 0 || self.opened.len() == valves.len()
    }

    fn you_moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        let mut nexts = vec![];
        let (dest, distance) = self.you;

        if distance > 0 {
            let mut next = self.clone();
            next.you = (dest, distance - 1);
            nexts.push((next, 0));
            return nexts;
        }

        if !self.opened(dest) {
            let mut next = self.clone();
            next.opened.insert(dest);
            let cost = next.remaining * valves[&dest].flow;
            nexts.push((next, -cost));
        }

        for (next_dest, next_distance) in &valves[&dest].exit {
            let mut next = self.clone();
            next.you = (*next_dest, next_distance - 1);
            nexts.push((next, 0));
        }

        nexts
    }

    fn elephant_moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        let mut nexts = vec![];
        let (dest, distance) = self.elephant;

        if distance > 0 {
            let mut next = self.clone();
            next.elephant = (dest, distance - 1);
            nexts.push((next, 0));
            return nexts;
        }

        if !self.opened(dest) {
            let mut next = self.clone();
            next.opened.insert(dest);
            let cost = next.remaining * valves[&dest].flow;
            nexts.push((next, -cost));
        }

        for (next_dest, next_distance) in &valves[&dest].exit {
            let mut next = self.clone();
            next.elephant = (*next_dest, next_distance - 1);
            nexts.push((next, 0));
        }

        nexts
    }

    fn moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        let mut moved = self.clone();
        moved.remaining -= 1;

        moved
            .you_moves(valves)
            .into_iter()
            .flat_map(|(n, c0)| {
                n.elephant_moves(valves)
                    .into_iter()
                    .map(move |(n, c1)| (n, c0 + c1))
            })
            .collect()
    }

    fn heuristic(&self, sorted_by_flow: &[(usize, i64)]) -> i64 {
        let mut h = 0;
        let mut r = self.remaining * 2;
        let mut i = 0;

        while r >= 1 && i < sorted_by_flow.len() {
            let inc = sorted_by_flow[i..]
                .iter()
                .position(|(name, _)| !self.opened(*name));

            if let Some(inc) = inc {
                i += inc;
            } else {
                break;
            }

            r -= 1;
            h += sorted_by_flow[i].1 * r;
            i += 1;
            r -= 1;
        }

        -h
    }
}

fn part_one(valves: &HashMap<&str, CompressedValve>) -> i64 {
    let mut sorted_by_flow: Vec<(&str, i64)> = valves.values().map(|v| (v.name, v.flow)).collect();
    sorted_by_flow.sort_by_key(|(_, f)| *f);
    sorted_by_flow.reverse();

    let (_, cost) = astar(
        &State::new(),
        |state| {
            let mut nexts = vec![];

            if !state.opened(state.current) {
                nexts.push(state.open(valves));
            }
            for (valve, cost) in &valves[state.current].exit {
                nexts.extend(state.move_to(valve, *cost));
            }

            nexts
        },
        |state| state.heuristic(&sorted_by_flow),
        |state| state.finished(valves),
    )
    .unwrap();

    -cost
}

fn part_two(valves: &HashMap<usize, IdValve>, start: usize) -> i64 {
    let mut sorted_by_flow: Vec<(usize, i64)> = valves.values().map(|v| (v.name, v.flow)).collect();
    sorted_by_flow.sort_by_key(|(_, f)| *f);
    sorted_by_flow.reverse();

    let (_, cost) = astar(
        &ElephantState::new(start),
        |state| state.moves(valves),
        |state| state.heuristic(&sorted_by_flow),
        |state| state.finished(valves),
    )
    .unwrap();

    -cost
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn example_part_one() {
        let valves = parse(INPUT);
        let compressed = compress(&valves);
        assert_eq!(part_one(&compressed), 1651);
    }

    #[test]
    fn example_part_two() {
        let valves = parse(INPUT);
        let compressed = compress(&valves);
        let (ids, start) = identifiers(&compressed);
        assert_eq!(part_two(&ids, start), 1707);
    }
}
