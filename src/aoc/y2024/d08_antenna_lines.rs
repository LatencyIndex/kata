// Puzzle description: https://adventofcode.com/2024/day/8

use crate::lgl::data::{array2d, index::Ix2s, table};
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

fn read_map(input: &str) -> Array2<char> {
    let input = table::read_rows_chars(input);
    array2d::to_ndarray(&input)
}

fn is_antenna(c: char) -> bool {
    c.is_ascii_digit() || c.is_ascii_alphabetic()
}

fn get_antennas(map: &Array2<char>) -> HashMap<char, Vec<Ix2s>> {
    let mut antennas: HashMap<char, Vec<Ix2s>> = HashMap::new();
    for (pos, c) in map.indexed_iter().filter(|(_pos, c)| is_antenna(**c)) {
        let pos: Ix2s = pos.try_into().unwrap();
        match antennas.get_mut(c) {
            Some(locations) => {
                locations.push(pos);
            }
            None => {
                antennas.insert(*c, vec![pos]);
            }
        }
    }
    antennas
}

fn pair_to_antinode(a: Ix2s, b: Ix2s) -> Ix2s {
    a + a - b
}

fn pair_to_all_antinodes(a: Ix2s, b: Ix2s, map: &Array2<char>) -> Vec<Ix2s> {
    let dr = a - b;
    let pos = (0..)
        .map(|k| a + dr * k)
        .take_while(|loc| array2d::contains(map, *loc));
    let neg = (1..)
        .map(|k| a - dr * k)
        .take_while(|loc| array2d::contains(map, *loc));
    pos.chain(neg).collect()
}

fn group_to_antinodes(locs: &[Ix2s]) -> Vec<Ix2s> {
    let mut nodes = Vec::new();
    for (i, pos_a) in locs.iter().enumerate() {
        for (j, pos_b) in locs.iter().enumerate() {
            if i != j {
                nodes.push(pair_to_antinode(*pos_a, *pos_b));
            }
        }
    }
    nodes
}

fn group_to_all_antinodes(locs: &[Ix2s], map: &Array2<char>) -> Vec<Ix2s> {
    let mut nodes = Vec::new();
    for (i, pos_a) in locs.iter().enumerate() {
        for (j, pos_b) in locs.iter().enumerate() {
            if i != j {
                nodes.append(&mut pair_to_all_antinodes(*pos_a, *pos_b, map));
            }
        }
    }
    nodes
}

const INPUT: &str = "data/y2024/d08/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let map = read_map(&input);
    let antennas = get_antennas(&map);
    let antinodes: HashSet<Ix2s> = antennas
        .iter()
        .flat_map(|(_key, locs)| group_to_antinodes(locs))
        .filter(|loc| array2d::contains(&map, *loc))
        .collect();
    antinodes.len()
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let map = read_map(&input);
    let antennas = get_antennas(&map);
    let antinodes: HashSet<Ix2s> = antennas
        .iter()
        .flat_map(|(_key, locs)| group_to_all_antinodes(locs, &map))
        .collect();
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 409);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1308);
    }
}
