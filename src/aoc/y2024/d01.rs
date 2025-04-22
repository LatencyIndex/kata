use std::{collections::HashMap, hash::Hash, path::Path};

use crate::lgl::data::table;

// Read text file as two columns of integers
fn read_two_cols(input: impl AsRef<Path>) -> (Vec<i64>, Vec<i64>) {
    let mut cols: Vec<Vec<i64>> = table::read_cols(input);
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

/// ```
/// use advent_of_code::aoc::y2024::d01::part1;
/// assert_eq!(part1(), 1938424);
/// ```
pub fn part1() -> i64 {
    let (l, r) = read_two_cols(INPUT);
    distance(l, r)
}

/// ```
/// use advent_of_code::aoc::y2024::d01::part2;
/// assert_eq!(part2(), 22014209);
/// ```
pub fn part2() -> i64 {
    let (l, r) = read_two_cols(INPUT);
    similarity(&l, r)
}
