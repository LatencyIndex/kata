// Puzzle description: https://adventofcode.com/2024/day/7

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut it = line.split(":");
    let result: u64 = it.next().unwrap().parse().unwrap();
    let vals: Vec<u64> = it
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (result, vals)
}

/// 0 bit is add, 1 is multiply.
fn eval(vals: &[u64], mut ops: u64) -> u64 {
    let mut result = vals[0];
    for x in &vals[1..] {
        if ops & 1 == 0 {
            result += x;
        } else {
            result *= x;
        }
        ops >>= 1;
    }
    result
}

fn concat(l: u64, r: u64) -> u64 {
    let r_digits = if r > 0 { r.ilog10() + 1 } else { 0 };
    l * 10u64.pow(r_digits) + r
}

/// Pretend ternary bits enumerate the operators.
/// 0 is add, 1 is multiply, 2 is concatenate.
fn eval_ternary(vals: &[u64], mut ops: u64) -> u64 {
    let mut result = vals[0];
    for x in &vals[1..] {
        match ops % 3 {
            0 => {
                result += x;
            }
            1 => {
                result *= x;
            }
            2 => {
                result = concat(result, *x);
            }
            _ => unreachable!(),
        }
        ops /= 3;
    }
    result
}

/// Returns the one after last valid sequence of +* operators
fn ops_ceil(vals: &[u64]) -> u64 {
    // Last valid sequence has len-1 bits set to 1.
    // Right-shift instead of len-1 to not overflow on len=0.
    (1 << vals.len()) >> 1
}

/// Returns the one after last valid sequence of +*concat operators
fn ceil_ternary(vals: &[u64]) -> u64 {
    // Last valid sequence is 3^nb_ops
    // nb_ops = len-1
    // Divide by 3 instead of len-1 to not underflow on len=0.
    (3u64.pow(vals.len() as u32)) / 3
}

fn has_valid_ops(result: u64, vals: &[u64]) -> bool {
    (0..ops_ceil(vals))
        .map(|ops| eval(vals, ops))
        .any(|x| x == result)
}

fn has_valid_ops_ternary(result: u64, vals: &[u64]) -> bool {
    (0..ceil_ternary(vals))
        .map(|ops| eval_ternary(vals, ops))
        .any(|x| x == result)
}

const INPUT: &str = "data/y2024/d07/input";

pub fn part1() -> u64 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let equations = input.lines().map(parse_line);
    let valid_eqs = equations.filter(|(result, vals)| has_valid_ops(*result, vals));
    valid_eqs.map(|(result, _vals)| result).sum()
}

pub fn part2() -> u64 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let equations = input.lines().map(parse_line);
    let valid_eqs = equations.filter(|(result, vals)| has_valid_ops_ternary(*result, vals));
    valid_eqs.map(|(result, _vals)| result).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 7710205485870);
    }
    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(), 20928985450275);
    }
}
