use std::fmt::Display;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
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

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(l) => write!(f, "[{}]", l.iter().join(",")),
            Packet::Int(i) => write!(f, "{}", i),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::List(l0), r0 @ Self::Int(..)) => l0.eq(&vec![r0.clone()]),
            (Self::Int(..), Self::List(..)) => other.eq(self),
        }
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
            (Packet::List(l0), r0 @ Packet::Int(_)) => l0.cmp(&vec![r0.clone()]),
            (Packet::Int(_), Packet::List(_)) => other.cmp(self).reverse(),
            (Packet::Int(l0), Packet::Int(r0)) => l0.cmp(r0),
        }
    }
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
    let (remain, pairs) = p_pairs(input).expect("valid input");
    if !remain.is_empty() {
        panic!("incomplete parse: {remain}");
    }
    pairs
}

fn p_pairs<'a>(input: &'a str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(
        tuple((line_ending, line_ending)),
        |input: &'a str| -> IResult<&'a str, (Packet, Packet)> {
            let (input, fst) = p_packet(input)?;
            let (input, _) = line_ending(input)?;
            let (input, snd) = p_packet(input)?;
            Ok((input, (fst, snd)))
        },
    )(input)
}

fn p_packet(input: &str) -> IResult<&str, Packet> {
    if input.starts_with('[') {
        let (input, packets) =
            delimited(tag("["), separated_list0(tag(","), p_packet), tag("]"))(input)?;
        Ok((input, Packet::List(packets)))
    } else {
        let (input, digits) = digit1(input)?;
        let int = digits.parse().unwrap();
        Ok((input, Packet::Int(int)))
    }
}

fn part_one(pairs: &[(Packet, Packet)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (fst, snd))| if fst < snd { Some(i + 1) } else { None })
        .sum()
}

fn dividers() -> (Packet, Packet) {
    let fst = p_packet("[[2]]").unwrap().1;
    let snd = p_packet("[[6]]").unwrap().1;
    (fst, snd)
}

fn part_two(pairs: &[(Packet, Packet)]) -> usize {
    let mut packets: Vec<Packet> = pairs
        .iter()
        .flat_map(|(fst, snd)| [fst.clone(), snd.clone()])
        .collect();
    let (fst, snd) = dividers();
    packets.push(fst.clone());
    packets.push(snd.clone());

    packets.sort();

    let fst_index = packets.iter().position(|p| p == &fst).unwrap();
    let snd_index = packets.iter().position(|p| p == &snd).unwrap();

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
