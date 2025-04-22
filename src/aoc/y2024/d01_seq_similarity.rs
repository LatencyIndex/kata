// Puzzle description: https://adventofcode.com/2024/day/1

use crate::lgl::data::table;
use std::{collections::HashMap, hash::Hash};

// Read text file as two columns of integers
fn read_two_cols(input: &str) -> (Vec<i64>, Vec<i64>) {
    let rows: Vec<Vec<i64>> = table::parse(table::read_rows_whitespace(input));
    let mut cols = table::transpose(rows);
    let r = cols.pop().unwrap();
    let l = cols.pop().unwrap();
    (l, r)
}

fn distance(mut l: Vec<i64>, mut r: Vec<i64>) -> i64 {
    l.sort();
    r.sort();
    l.iter().zip(r).map(|(x, y)| (x - y).abs()).sum()
}

// Number of occurences of each value in v
fn get_counts<T: Eq + Hash>(v: Vec<T>) -> HashMap<T, i64> {
    let mut counts = HashMap::new();
    for x in v {
        match counts.get_mut(&x) {
            Some(c) => {
                *c += 1;
            }
            None => {
                counts.insert(x, 1);
            }
        }
    }
    counts
}

fn similarity(l: &[i64], r: Vec<i64>) -> i64 {
    let counts = get_counts(r);
    l.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum()
}

const INPUT: &str = "data/y2024/d01/input";

pub fn part1() -> i64 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (l, r) = read_two_cols(&input);
    distance(l, r)
}

pub fn part2() -> i64 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (l, r) = read_two_cols(&input);
    similarity(&l, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1938424);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 22014209);
    }
}
