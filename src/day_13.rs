use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64 as nom_i64, line_ending, space0},
    combinator::all_consuming,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult, Parser,
};

pub fn solution(input: &str) -> String {
    let pairs = parse(input);
    format!("{}, {}", part_one(&pairs), part_two(&pairs))
}

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(i64),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(l0), Packet::List(r0)) => l0.cmp(r0),
            (Packet::List(l0), Packet::Int(r0)) => l0.as_slice().cmp(&[Packet::Int(*r0)]),
            (Packet::Int(_), Packet::List(_)) => other.cmp(self).reverse(),
            (Packet::Int(l0), Packet::Int(r0)) => l0.cmp(r0),
        }
    }
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
    all_consuming(p_pairs)(input).expect("valid input").1
}

fn p_pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(tuple((line_ending, line_ending)), p_pair)(input)
}

fn p_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(p_packet, line_ending, p_packet)(input)
}

fn p_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(
            tag("["),
            separated_list0(tag(",").and(space0), p_packet),
            tag("]"),
        )
        .map(Packet::List),
        nom_i64.map(Packet::Int),
    ))(input)
}

fn part_one(pairs: &[(Packet, Packet)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (fst, snd))| if fst < snd { Some(i + 1) } else { None })
        .sum()
}

fn part_two(pairs: &[(Packet, Packet)]) -> usize {
    let mut packets: Vec<&Packet> = pairs.iter().flat_map(|(fst, snd)| [fst, snd]).collect();
    let fst = p_packet("[[2]]").unwrap().1;
    let snd = p_packet("[[6]]").unwrap().1;
    packets.extend([&fst, &snd]);

    packets.sort_unstable();

    let fst_index = packets.binary_search(&&fst).unwrap();
    let snd_index = packets.binary_search(&&snd).unwrap();
    (fst_index + 1) * (snd_index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn example_part_one() {
        let pairs = parse(INPUT);
        assert_eq!(part_one(&pairs), 13);
    }

    #[test]
    fn example_part_two() {
        let pairs = parse(INPUT);
        assert_eq!(part_two(&pairs), 140);
    }
}
