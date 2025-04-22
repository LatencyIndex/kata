use std::ops::Sub;

use crate::lgl::data::table;

fn diff<T: Copy + Sub<Output = T>>(v: &[T]) -> Vec<T> {
    v.windows(2).map(|x| x[1] - x[0]).collect()
}

fn is_monotonic(dxs: &[i64]) -> bool {
    dxs.iter().all(|&x| x >= 0) || dxs.iter().all(|&x| x <= 0)
}

// Adjacent values differ by at least 1 and at most 3,
// i.e. all the abs deltas are in [1,3] range
fn is_bounded(dxs: &[i64]) -> bool {
    dxs.iter().all(|x| (1..=3).contains(&x.abs()))
}

fn is_safe(xs: &[i64]) -> bool {
    let dxs = diff(xs);
    is_monotonic(&dxs) && is_bounded(&dxs)
}

fn remove_ix<T>(rx: usize, xs: Vec<T>) -> Vec<T> {
    xs.into_iter()
        .enumerate()
        .filter(|(i, _)| *i != rx)
        .map(|(_, x)| x)
        .collect()
}

fn is_tolerable(xs: &[i64]) -> bool {
    (0..xs.len())
        .map(|i| remove_ix(i, xs.to_vec()))
        .any(|v| is_safe(&v))
}

const INPUT: &str = "data/y2024/d02/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let table: Vec<Vec<i64>> = table::parse(table::read_rows_whitespace(&input));
    table.iter().filter(|xs| is_safe(xs)).count()
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let table: Vec<Vec<i64>> = table::parse(table::read_rows_whitespace(&input));
    table
        .iter()
        .filter(|xs| is_safe(xs) || is_tolerable(xs))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 379);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 430);
    }
}
