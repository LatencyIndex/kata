// Puzzle description: https://adventofcode.com/2024/day/22

use std::collections::HashMap;

fn next_secret(mut i: i64) -> i64 {
    i ^= i * 64;
    i %= 16777216;
    i ^= i / 32;
    i %= 16777216;
    i ^= i * 2048;
    i %= 16777216;
    i
}

// 0th secret number is the input secret unchanged.
fn nth_secret(secret: i64, n: usize) -> i64 {
    (0..n).fold(secret, |acc, _| next_secret(acc))
}

fn get_prices(mut secret: i64, n: usize) -> Vec<i64> {
    let mut v = Vec::with_capacity(n);
    while v.len() < n {
        v.push(secret % 10);
        secret = next_secret(secret);
    }
    v
}

fn get_deltas(v: &[i64]) -> Vec<i64> {
    v.windows(2).map(|w| w[1] - w[0]).collect()
}

// Map from change sequence, to the price it would yield.
type SeqValues = HashMap<Vec<i64>, i64>;

// What price any chosen sequence of deltas would yield.
fn get_seq_values(prices: &[i64]) -> SeqValues {
    let seq_len = 4;
    let deltas = get_deltas(prices);
    let mut seq_vals = HashMap::new();
    for (deltas, price) in deltas.windows(seq_len).zip(&prices[seq_len..]) {
        // In case of duplicate keys, HashMap::from_iter overwrites earlier
        // items with later ones, so we do it explicitly.
        if !seq_vals.contains_key(deltas) {
            seq_vals.insert(deltas.to_vec(), *price);
        }
    }
    seq_vals
}

fn sum_union(dst: &mut SeqValues, src: SeqValues) {
    for (k, v) in src {
        dst.entry(k).and_modify(|v0| *v0 += v).or_insert(v);
    }
}

const INPUT: &str = "data/y2024/d22/input";

pub fn part1() -> i64 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    input
        .lines()
        .map(|s| nth_secret(s.parse().unwrap(), 2000))
        .sum()
}

pub fn part2() -> i64 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let mut seq_vals = HashMap::new();
    for s in input.lines() {
        // We will have 2000 price _changes_, i.e. 2001 prices total.
        let prices = get_prices(s.parse().unwrap(), 2001);
        sum_union(&mut seq_vals, get_seq_values(&prices));
    }
    *seq_vals.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 19854248602);
    }
    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(), 2223);
    }
}
