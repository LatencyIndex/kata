use regex::Regex;
use std::{collections::HashMap, str::FromStr};

fn parse_val(s: &str) -> (&str, u64) {
    let (var, val) = s.split_once(':').unwrap();
    (var, val.trim().parse().unwrap())
}

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Op::And),
            "OR" => Ok(Op::Or),
            "XOR" => Ok(Op::Xor),
            x => Err(format!("cannot convert {x} to Op")),
        }
    }
}

#[derive(Debug)]
struct Expr<'a> {
    l: &'a str,
    op: Op,
    r: &'a str,
    out: &'a str,
}

impl Expr<'_> {
    fn eval(&self, vals: &HashMap<&str, u64>) -> Option<u64> {
        if let (Some(l), Some(r)) = (vals.get(self.l), vals.get(self.r)) {
            match self.op {
                Op::And => Some(l & r),
                Op::Or => Some(l | r),
                Op::Xor => Some(l ^ r),
            }
        } else {
            None
        }
    }
}

fn parse_expr(s: &str) -> Expr {
    let mut words = s.split_whitespace();
    let l = words.next().unwrap();
    let op = words.next().unwrap().parse().unwrap();
    let r = words.next().unwrap();
    let _arrow = words.next().unwrap();
    let out = words.next().unwrap();
    Expr { l, op, r, out }
}

fn parse_input(input: &str) -> (HashMap<&str, u64>, Vec<Expr>) {
    let mut blocks = input.split("\n\n");
    let vals = blocks.next().unwrap();
    let exprs = blocks.next().unwrap();
    let vals = vals.lines().map(parse_val).collect();
    let exprs = exprs.lines().map(parse_expr).collect();
    (vals, exprs)
}

fn eval<'a>(vals: &mut HashMap<&'a str, u64>, mut exprs: Vec<Expr<'a>>) {
    // It would probably be best to represent the expressions as a graph, but this will suffice.
    while !exprs.is_empty() {
        // Evaluate expressions that have inputs
        for expr in exprs.iter() {
            if let Some(out) = expr.eval(vals) {
                vals.insert(expr.out, out);
            }
        }
        // Remove evaluated expressions.
        exprs.retain(|e| !vals.contains_key(e.out));
    }
}

fn read_z(vals: &HashMap<&str, u64>) -> u64 {
    let re = Regex::new(r"^z(\d\d)$").unwrap();
    vals.iter()
        .filter_map(|(k, v)| {
            re.captures(k).map(|c| {
                let i: u32 = c[1].parse().unwrap();
                v * 2u64.pow(i)
            })
        })
        .sum()
}

const INPUT: &str = "data/y2024/d24/input";

pub fn part1() -> u64 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (mut vals, exprs) = parse_input(&input);
    eval(&mut vals, exprs);
    read_z(&vals)
}

pub fn part2() -> usize {
    // let input = std::fs::read_to_string(INPUT).unwrap();
    2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 41324968993486);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2);
    }
}
