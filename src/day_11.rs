use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, one_of, space1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

pub fn solution(input: &str) -> String {
    let monkeys = parse(input);
    format!("{}, {}", part_one(&monkeys), part_two(&monkeys))
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
enum Rhs {
    Imm(i64),
    Old,
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    op: BinOp,
    rhs: Rhs,
}

impl Operation {
    fn apply(self, worry: i64) -> i64 {
        let r = match self.rhs {
            Rhs::Imm(i) => i,
            Rhs::Old => worry,
        };

        match self.op {
            BinOp::Add => worry + r,
            BinOp::Mul => worry * r,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
enum TestOp {
    Divisible(i64),
}

#[derive(Debug, Clone, Copy)]
struct Test {
    op: TestOp,
    if_true: i64,
    if_false: i64,
}

impl Test {
    fn apply(self, worry: i64) -> i64 {
        let satisfy = match self.op {
            TestOp::Divisible(arg) => worry % arg == 0,
        };

        if satisfy {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    _id: i64,
    items: Vec<i64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn inspect(&self, worry: i64) -> i64 {
        self.operation.apply(worry)
    }

    fn throw(&self, worry: i64) -> i64 {
        self.test.apply(worry)
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    let (remain, monkeys) = p_monkeys(input).expect("valid input");
    if !remain.is_empty() {
        panic!("incomplete parse: {}", remain);
    }
    for (i, m) in monkeys.iter().enumerate() {
        assert_eq!(m._id, i as i64)
    }

    monkeys
}

fn p_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), p_monkey)(input)
}

fn p_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = delimited(tag("Monkey "), p_unsigned, tag(":\n"))(input)?;

    let (input, items) = delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), p_unsigned),
        newline,
    )(input)?;
    let (input, operation) =
        delimited(tag("  Operation: new = old "), p_operation, newline)(input)?;
    let (input, testop) = delimited(tag("  Test: "), p_testop, newline)(input)?;
    let (input, if_true) =
        delimited(tag("    If true: throw to monkey "), p_unsigned, newline)(input)?;
    let (input, if_false) = preceded(tag("    If false: throw to monkey "), p_unsigned)(input)?;

    let test = Test {
        op: testop,
        if_true,
        if_false,
    };
    let monkey = Monkey {
        _id: id,
        items,
        operation,
        test,
    };

    Ok((input, monkey))
}

fn p_operation(input: &str) -> IResult<&str, Operation> {
    let (input, op) = one_of("+*")(input)?;
    let (input, _) = space1(input)?;
    let (input, digits_or_old) = digit1.or(tag("old")).parse(input)?;
    let binop = match op {
        '+' => BinOp::Add,
        '*' => BinOp::Mul,
        _ => unreachable!("filtered by parser"),
    };
    let rhs = match digits_or_old {
        "old" => Rhs::Old,
        _ => Rhs::Imm(digits_or_old.parse().unwrap()),
    };
    let operation = Operation { op: binop, rhs };
    Ok((input, operation))
}

fn p_testop(input: &str) -> IResult<&str, TestOp> {
    let (input, op) = tag("divisible")(input)?;
    let (input, _) = tag(" by ")(input)?;
    let (input, arg) = p_unsigned(input)?;

    let testop = match op {
        "divisible" => TestOp::Divisible(arg),
        _ => unreachable!("filtered by parser"),
    };

    Ok((input, testop))
}

fn p_unsigned(input: &str) -> IResult<&str, i64> {
    let (input, unsigned) = digit1(input)?;
    Ok((input, unsigned.parse().unwrap()))
}

struct MonkeyState {
    inspected: i64,
    items: Vec<i64>,
}

impl MonkeyState {
    fn new(monkey: &Monkey) -> Self {
        Self {
            inspected: 0,
            items: monkey.items.clone(),
        }
    }
}

fn monkey_business(monkeys: &[Monkey], round: u32, relief: bool) -> i64 {
    let mut states: Vec<MonkeyState> = monkeys.iter().map(MonkeyState::new).collect();
    // all tests are divisibility by a prime number
    let modulo = monkeys
        .iter()
        .map(|m| {
            #[allow(unreachable_patterns)]
            match m.test.op {
                TestOp::Divisible(arg) => arg,
                _ => unreachable!("invalid hypothesis"),
            }
        })
        .unique()
        .product::<i64>();

    for _ in 0..round {
        for i in 0..states.len() {
            let item_len = states[i].items.len();
            for t in 0..item_len {
                let worry = states[i].items[t];
                let new_worry = {
                    let w = monkeys[i].inspect(worry);
                    if relief {
                        w / 3
                    } else {
                        w % modulo
                    }
                };
                let throw_to = monkeys[i].throw(new_worry) as usize;
                states[throw_to].items.push(new_worry);
            }
            states[i].inspected += item_len as i64;
            states[i].items = states[i].items.split_off(item_len);
        }
    }

    states.sort_by_key(|s| s.inspected);
    states[states.len() - 1].inspected * states[states.len() - 2].inspected
}

fn part_one(monkeys: &[Monkey]) -> i64 {
    monkey_business(monkeys, 20, true)
}

fn part_two(monkeys: &[Monkey]) -> i64 {
    monkey_business(monkeys, 10000, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn example_part_one() {
        let monkeys = parse(INPUT);
        assert_eq!(part_one(&monkeys), 10605);
    }

    #[test]
    fn example_part_two() {
        let monkeys = parse(INPUT);
        assert_eq!(part_two(&monkeys), 2713310158);
    }
}
