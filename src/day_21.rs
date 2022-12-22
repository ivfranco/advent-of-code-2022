use std::{collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{i64, line_ending},
    character::complete::{one_of, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult, Parser,
};

pub fn solution(input: &str) -> String {
    let yells = parse(input);
    format!("{}, {}", part_one(&yells), part_two(&yells))
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    fn apply(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs,
        }
    }

    fn lhs_should_be(self, rhs: i64, result: i64) -> i64 {
        match self {
            Add => result - rhs,
            Sub => result + rhs,
            Mul => result / rhs,
            Div => result * rhs,
        }
    }

    fn rhs_should_be(self, lhs: i64, result: i64) -> i64 {
        match self {
            Add => result - lhs,
            Sub => lhs - result,
            Mul => result / lhs,
            // could be problematic because of remainder
            Div => lhs / result,
        }
    }
}

use pathfinding::prelude::topological_sort;
use BinOp::*;

#[derive(Clone, Copy)]
struct Expr<'a> {
    op: BinOp,
    lhs: &'a str,
    rhs: &'a str,
}

#[derive(Clone, Copy)]
enum Job<'a> {
    Expr(Expr<'a>),
    Number(i64),
}

struct Yell<'a> {
    monkey: &'a str,
    job: Job<'a>,
}

fn parse(input: &str) -> Vec<Yell> {
    let (_, yells) =
        all_consuming(separated_list1(line_ending, p_yell))(input).expect("valid complete parse");
    yells
}

fn p_monkey(input: &str) -> IResult<&str, &str> {
    take_while_m_n(4, 4, |c: char| c.is_ascii_lowercase())(input)
}

fn p_expr(input: &str) -> IResult<&str, Expr> {
    let (input, lhs) = p_monkey(input)?;
    let (input, op_char) = delimited(space1, one_of("+-*/"), space1)(input)?;
    let (input, rhs) = p_monkey(input)?;

    let op = match op_char {
        '+' => Add,
        '-' => Sub,
        '*' => Mul,
        '/' => Div,
        _ => unreachable!("filtered by parser"),
    };

    Ok((input, Expr { op, lhs, rhs }))
}

fn p_job(input: &str) -> IResult<&str, Job> {
    alt((p_expr.map(Job::Expr), i64.map(Job::Number)))(input)
}

fn p_yell(input: &str) -> IResult<&str, Yell> {
    let (input, monkey) = terminated(p_monkey, tag(": "))(input)?;
    let (input, job) = p_job(input)?;
    Ok((input, Yell { monkey, job }))
}

fn toposort<'a>(deps: &'a HashMap<&str, Job>) -> Vec<&'a str> {
    let mut sorted = topological_sort(&["root"], |monkey| {
        let mut successors = vec![];
        if let Job::Expr(expr) = deps[monkey] {
            successors.extend([expr.lhs, expr.rhs]);
        }
        successors
    })
    .expect("no cycle");
    sorted.reverse();
    sorted
}

fn part_one(yells: &[Yell]) -> i64 {
    let deps: HashMap<&str, Job> = yells.iter().map(|y| (y.monkey, y.job)).collect();
    let mut values: HashMap<&str, i64> = HashMap::new();
    let sorted = toposort(&deps);

    for monkey in sorted {
        let v = match deps[monkey] {
            Job::Expr(expr) => {
                let lhs = values[expr.lhs];
                let rhs = values[expr.rhs];
                expr.op.apply(lhs, rhs)
            }
            Job::Number(n) => n,
        };
        values.insert(monkey, v);
    }

    values["root"]
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Number(i64),
    You,
}

#[derive(Debug, Clone)]
enum Nested {
    Expr {
        op: BinOp,
        lhs: Rc<Nested>,
        rhs: Rc<Nested>,
    },
    Value(Value),
}

impl Nested {
    fn you_should_yell(&self) -> i64 {
        let (mut curr, mut equal) = match self {
            Nested::Expr { lhs, rhs, .. } => match (lhs.as_ref(), rhs.as_ref()) {
                (expr @ Nested::Expr { .. }, Nested::Value(Value::Number(n)))
                | (Nested::Value(Value::Number(n)), expr @ Nested::Expr { .. }) => (expr, *n),
                _ => panic!("should be rejected by hypothesis test"),
            },
            _ => panic!("should be rejected by hypothesis test"),
        };

        while let Nested::Expr { op, lhs, rhs } = curr {
            match (lhs.as_ref(), rhs.as_ref()) {
                (expr @ Nested::Expr { .. }, Nested::Value(Value::Number(n))) => {
                    equal = op.lhs_should_be(*n, equal);
                    curr = expr;
                }
                (Nested::Value(Value::Number(n)), expr @ Nested::Expr { .. }) => {
                    equal = op.rhs_should_be(*n, equal);
                    curr = expr;
                }
                (Nested::Value(Value::Number(n)), Nested::Value(Value::You)) => {
                    return op.rhs_should_be(*n, equal);
                }
                (Nested::Value(Value::You), Nested::Value(Value::Number(n))) => {
                    return op.lhs_should_be(*n, equal);
                }
                _ => panic!("should be rejected by hypothesis test"),
            }
        }

        unreachable!("should terminate in the loop")
    }
}

/// On every level of the final nested expression of monkey "root", either lhs or rhs can be reduced
/// to a number.
fn hypothesis(nested: &Nested) -> bool {
    let mut curr = nested;

    while let Nested::Expr { lhs, rhs, .. } = curr {
        match (lhs.as_ref(), rhs.as_ref()) {
            (Nested::Value(Value::Number(..)), expr @ Nested::Expr { .. })
            | (expr @ Nested::Expr { .. }, Nested::Value(Value::Number(..))) => curr = expr,
            (Nested::Value(Value::Number(..)), Nested::Value(..))
            | (Nested::Value(..), Nested::Value(Value::Number(..))) => return true,
            _ => return false,
        }
    }

    unreachable!("should terminate in the loop")
}

fn part_two(yells: &[Yell]) -> i64 {
    let deps: HashMap<&str, Job> = yells.iter().map(|y| (y.monkey, y.job)).collect();
    let mut values: HashMap<&str, Rc<Nested>> = HashMap::new();

    for monkey in toposort(&deps) {
        let v = match &deps[monkey] {
            Job::Expr(expr) => match (&*values[expr.lhs], &*values[expr.rhs]) {
                (Nested::Value(Value::Number(lhs)), Nested::Value(Value::Number(rhs))) => {
                    Nested::Value(Value::Number(expr.op.apply(*lhs, *rhs)))
                }
                _ => Nested::Expr {
                    op: expr.op,
                    lhs: Rc::clone(&values[expr.lhs]),
                    rhs: Rc::clone(&values[expr.rhs]),
                },
            },
            Job::Number(n) => {
                if monkey == "humn" {
                    Nested::Value(Value::You)
                } else {
                    Nested::Value(Value::Number(*n))
                }
            }
        };

        values.insert(monkey, Rc::new(v));
    }

    assert!(hypothesis(&values["root"]));
    values["root"].you_should_yell()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn example_part_one() {
        let yells = parse(INPUT);
        assert_eq!(part_one(&yells), 152);
    }

    #[test]
    fn example_part_two() {
        let yells = parse(INPUT);
        assert_eq!(part_two(&yells), 301);
    }
}
