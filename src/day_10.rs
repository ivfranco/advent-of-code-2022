use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{line_ending, space1},
    multi::separated_list1,
    IResult, Parser,
};

pub fn solution(input: &str) -> String {
    let insts = parse(input);
    format!("{}\n{}", part_one(&insts), part_two(&insts))
}

#[derive(Clone, Copy)]
enum Inst {
    Noop,
    Addx(i64),
}

impl Inst {
    fn cycle(self) -> i64 {
        match self {
            Inst::Noop => 1,
            Inst::Addx(_) => 2,
        }
    }
}

fn parse(input: &str) -> Vec<Inst> {
    let (remain, insts) = p_insts(input).expect("invalid input");
    if !remain.is_empty() {
        panic!("incomplete parse");
    }
    insts
}

fn p_insts(input: &str) -> IResult<&str, Vec<Inst>> {
    separated_list1(line_ending, p_inst)(input)
}

fn p_inst(input: &str) -> IResult<&str, Inst> {
    let (input, op) = tag("noop").or(tag("addx")).parse(input)?;
    match op {
        "noop" => Ok((input, Inst::Noop)),
        "addx" => {
            let (input, _) = space1(input)?;
            let (input, imm) = take_while(|c: char| c != '\n')(input)?;
            let inst = Inst::Addx(imm.parse().expect("invalid immediate"));
            Ok((input, inst))
        }
        _ => unreachable!("invalid op"),
    }
}

struct Circuit {
    cycle: i64,
    x: i64,
    rip: usize,
    ic: i64,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            cycle: 1,
            x: 1,
            rip: 0,
            ic: 0,
        }
    }

    fn apply(&mut self, inst: Inst) {
        match inst {
            Inst::Noop => (),
            Inst::Addx(dx) => self.x += dx,
        }
    }

    fn stop_at(&mut self, insts: &[Inst], stop: i64) {
        while self.cycle + (insts[self.rip].cycle() - self.ic) <= stop {
            self.apply(insts[self.rip]);
            self.cycle += insts[self.rip].cycle() - self.ic;
            self.rip += 1;
            self.ic = 0;
        }

        if self.cycle < stop {
            self.ic = stop - self.cycle;
            self.cycle = stop;
        }
    }
}

fn part_one(insts: &[Inst]) -> i64 {
    let mut circuit = Circuit::new();

    (20..=220)
        .step_by(40)
        .map(|stop| {
            circuit.stop_at(insts, stop);
            circuit.cycle * circuit.x
        })
        .sum()
}

fn part_two(insts: &[Inst]) -> String {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    let mut circuit = Circuit::new();
    let mut crt = String::with_capacity((WIDTH + 1) * HEIGHT);

    for y in 0..HEIGHT {
        let mut row = vec![b'.'; WIDTH];
        #[allow(clippy::needless_range_loop)]
        for x in 0..WIDTH {
            let stop = (y * WIDTH + x + 1) as i64;
            circuit.stop_at(insts, stop);
            if (circuit.x - x as i64).abs() <= 1 {
                row[x] = b'#';
            }
        }
        crt.push_str(std::str::from_utf8(&row).unwrap());
        crt.push('\n');
    }

    crt
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn example_part_one() {
        let insts = parse(INPUT);
        assert_eq!(part_one(&insts), 13140);
    }

    #[test]
    fn example_part_two() {
        const CRT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        let insts = parse(INPUT);
        assert_eq!(part_two(&insts), CRT);
    }
}
