// Puzzle description: https://adventofcode.com/2024/day/11

use std::collections::HashMap;

fn parse_stones(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn nb_digits(i: u64) -> u32 {
    if i == 0 {
        1
    } else {
        i.ilog10() + 1
    }
}

fn split(i: u64, rdigits: u32) -> Vec<u64> {
    let n = 10u64.pow(rdigits);
    vec![i / n, i % n]
}

fn get_children(number: u64) -> Vec<u64> {
    if number == 0 {
        vec![1]
    } else {
        let nb_digits = nb_digits(number);
        if nb_digits % 2 == 0 {
            split(number, nb_digits / 2)
        } else {
            vec![2024 * number]
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Stone {
    number: u64,
    blinks_left: usize,
}

impl Stone {
    fn children(&self) -> Vec<Stone> {
        if 0 < self.blinks_left {
            get_children(self.number)
                .into_iter()
                .map(|number| Stone {
                    number,
                    blinks_left: self.blinks_left - 1,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Default)]
struct Cache {
    cached: HashMap<Stone, usize>,
}

impl Cache {
    fn insert(&mut self, key: Stone, val: usize) {
        self.cached.insert(key, val);
    }
    fn get(&self, key: &Stone) -> Option<usize> {
        // Special-case small numbers of blinks for efficiency
        if key.blinks_left == 0 {
            Some(1)
        } else if key.blinks_left == 1 {
            Some(key.children().len())
        } else {
            self.cached.get(key).copied()
        }
    }
}

fn count_descendants(stone: &Stone, cache: &mut Cache) -> usize {
    if let Some(n) = cache.get(stone) {
        n
    } else {
        let n = stone
            .children()
            .iter()
            .map(|stone| count_descendants(stone, cache))
            .sum();
        cache.insert(stone.clone(), n);
        n
    }
}

fn count_stones(stone_nbs: &[u64], blinks_left: usize) -> usize {
    let stones = stone_nbs.iter().map(|&number| Stone {
        number,
        blinks_left,
    });
    let mut cache = Cache::default();
    stones
        .map(|stone| count_descendants(&stone, &mut cache))
        .sum()
}

const INPUT: &str = "data/y2024/d11/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let stone_nbs = parse_stones(&input);
    count_stones(&stone_nbs, 25)
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let stone_nbs = parse_stones(&input);
    count_stones(&stone_nbs, 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 202019);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 239321955280205);
    }
}
