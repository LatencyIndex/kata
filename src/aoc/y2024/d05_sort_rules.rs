// Puzzle description: https://adventofcode.com/2024/day/5

use crate::lgl::data::table;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

type Successors = HashMap<u32, HashSet<u32>>;

fn parse_input(input: &str) -> (Successors, Vec<Vec<u32>>) {
    let mut inputs = input.split("\n\n");

    let precedences = inputs.next().unwrap();
    let precedences = table::read_rows(precedences, "|");
    let precedences: Vec<Vec<u32>> = table::parse(precedences);

    let mut successors: Successors = HashMap::new();
    for x in precedences {
        match successors.get_mut(&x[0]) {
            Some(succ) => drop(succ.insert(x[1])),
            None => {
                let mut succ = HashSet::new();
                succ.insert(x[1]);
                successors.insert(x[0], succ);
            }
        }
    }

    let sequences = inputs.next().unwrap();
    let sequences = table::read_rows(sequences, ",");
    let sequences: Vec<Vec<u32>> = table::parse(sequences);

    (successors, sequences)
}

fn successor_is_first(successors: &Successors, x0: &u32, x1: &u32) -> bool {
    successors
        .get(x1)
        .map(|my_succ| my_succ.contains(x0))
        .unwrap_or(false)
}

fn fails(successors: &Successors, xs: &[u32]) -> bool {
    xs.iter().enumerate().any(|(i, x1)| {
        let x0s = &xs[..i];
        x0s.iter().any(|x0| successor_is_first(successors, x0, x1))
    })
}

const INPUT: &str = "data/y2024/d05/input";

pub fn part1() -> u32 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (successors, sequences) = parse_input(&input);
    sequences
        .iter()
        .filter(|seq| !fails(&successors, seq))
        .map(|seq| seq[seq.len() / 2])
        .sum()
}

pub fn part2() -> u32 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (successors, mut sequences) = parse_input(&input);

    let cmp = |l: &u32, r: &u32| -> Ordering {
        if successor_is_first(&successors, l, r) {
            Ordering::Greater
        } else if successor_is_first(&successors, r, l) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };

    sequences
        .iter_mut()
        .filter(|seq| fails(&successors, seq))
        .map(|seq| {
            seq.sort_by(cmp);
            seq[seq.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 5208);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 6732);
    }
}
