// Puzzle description: https://adventofcode.com/2024/day/12

use crate::lgl::data::{
    array2d::{self, Array2},
    index::Ix2s,
    table,
};
use std::{collections::HashSet, fmt::Debug};

enum Pixel {
    // Unprocessed pixels
    Val(char),
    // Pixels currently being processed
    Flooded,
    // Already processed pixels
    Consumed,
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: char = match self {
            Pixel::Val(c) => *c,
            Pixel::Flooded => '*',
            Pixel::Consumed => '.',
        };
        write!(f, "{s}")
    }
}

impl Pixel {
    fn unwrap(&self) -> char {
        if let Pixel::Val(x) = self {
            *x
        } else {
            panic!("tried to unwrap a Pixel without value");
        }
    }
    fn is_flooded(&self) -> bool {
        matches!(self, Self::Flooded)
    }
}

// All adjacent indexes, including negative and outside the bounds of any map.
fn get_adjacent(pos: Ix2s) -> Vec<Ix2s> {
    let drs = [Ix2s(0, 1), Ix2s(0, -1), Ix2s(1, 0), Ix2s(-1, 0)];
    drs.into_iter().map(|dr| pos + dr).collect()
}

// Returns the indices of all flooded pixels.
fn flood_region(garden: &mut Array2<Pixel>, origin: Ix2s) -> Vec<Ix2s> {
    let val = array2d::get(garden, origin).unwrap().unwrap();
    // Flood the origin:
    let mut flooded = vec![origin];
    *array2d::get_mut(garden, origin).unwrap() = Pixel::Flooded;
    // Flood-fill this region of the map
    let mut i = 0;
    while let Some(pos) = flooded.get(i).copied() {
        i += 1;
        // Flood same-value adjacent pixels
        for adjacent_pos in get_adjacent(pos) {
            match array2d::get(garden, adjacent_pos) {
                Some(Pixel::Val(pixel_val)) if *pixel_val == val => {
                    *array2d::get_mut(garden, adjacent_pos).unwrap() = Pixel::Flooded;
                    flooded.push(adjacent_pos);
                }
                _ => {}
            }
        }
    }
    flooded
}

// Perimeter of a single flooded pixel.
// I.e. the number of sides facing unflooded pixels, including out-of-bounds 'pixels'.
fn one_perimeter(garden: &Array2<Pixel>, flooded: Ix2s) -> usize {
    get_adjacent(flooded)
        .into_iter()
        .map(|adjacent| match array2d::get(garden, adjacent) {
            Some(Pixel::Flooded) => 0,
            _ => 1,
        })
        .sum()
}

fn total_perimeter(garden: &Array2<Pixel>, flooded: &[Ix2s]) -> usize {
    flooded.iter().map(|r| one_perimeter(garden, *r)).sum()
}

fn consume_flooded(garden: &mut Array2<Pixel>, flooded: &[Ix2s]) {
    for r in flooded {
        *array2d::get_mut(garden, *r).unwrap() = Pixel::Consumed;
    }
}

fn consume_cost_of_region(garden: &mut Array2<Pixel>, origin: Ix2s) -> usize {
    let flooded = flood_region(garden, origin);
    let area = flooded.len();
    let perimeter = total_perimeter(garden, &flooded);
    let cost = area * perimeter;
    consume_flooded(garden, &flooded);
    cost
}

// Number of vertexes associated with a position.
// Because diagonally-adjacent vertexes are not merged,
// such a position counts as 2 vertexes, not 1.
// Position is not a tile position, but a vertex position.
// A NxM grid of tiles has (N+1)x(M+1) vertexes, i.e. each tile has 4 vertexes.
fn count_one_pixel_vertexes(garden: &Array2<Pixel>, pos: Ix2s) -> usize {
    let is_flooded =
        |pos: Ix2s| -> bool { array2d::get(garden, pos).map_or(false, |p| p.is_flooded()) };
    let drs = [Ix2s(0, 0), Ix2s(0, -1), Ix2s(-1, 0), Ix2s(-1, -1)];
    let flooded: [bool; 4] = drs.map(|dr| is_flooded(pos + dr));
    let nb_flooded: usize = flooded.map(|b| b as usize).iter().sum();
    if nb_flooded == 1 || nb_flooded == 3 {
        // Is vertex if it has 1 or 3 adjacents flooded,
        1
    } else if nb_flooded == 2 {
        // Or if it has 2 flooded diagonally.
        let is_vert = flooded[0] == flooded[3];
        2 * (is_vert as usize)
    } else {
        0
    }
}

fn get_adjacent_vertex_indexes(tile_index: Ix2s) -> [Ix2s; 4] {
    let drs = [Ix2s(0, 0), Ix2s(0, 1), Ix2s(1, 0), Ix2s(1, 1)];
    drs.map(|dr| tile_index + dr)
}

// All the unique vertex indexes adjacent to these tiles.
// There is a more elegant way of counting sides during flood-fill, but this also works.
fn get_vertex_indexes(tiles: &[Ix2s]) -> Vec<Ix2s> {
    let s: HashSet<Ix2s> = tiles
        .iter()
        .flat_map(|pos| get_adjacent_vertex_indexes(*pos))
        .collect();
    s.into_iter().collect()
}

// Vertexes of the flooded region.
fn count_vertexes(garden: &Array2<Pixel>, flooded: &[Ix2s]) -> usize {
    get_vertex_indexes(flooded)
        .into_iter()
        .map(|pos| count_one_pixel_vertexes(garden, pos))
        .sum()
}

fn consume_discounted_cost_of_region(garden: &mut Array2<Pixel>, origin: Ix2s) -> usize {
    let flooded = flood_region(garden, origin);
    let area = flooded.len();
    let sides = count_vertexes(garden, &flooded);
    let cost = area * sides;
    consume_flooded(garden, &flooded);
    cost
}

fn find_next_region(garden: &Array2<Pixel>, indexes: &mut Vec<Ix2s>) -> Option<Ix2s> {
    while let Some(pos) = indexes.pop() {
        if let Some(Pixel::Val(_)) = array2d::get(garden, pos) {
            return Some(pos);
        }
    }
    None
}

const INPUT: &str = "data/y2024/d12/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let garden = array2d::to_ndarray(&table::read_rows_chars(&input));
    let mut garden = garden.map(|x| Pixel::Val(*x));
    let mut indexes: Vec<Ix2s> = garden
        .indexed_iter()
        .map(|(i, _)| Ix2s::try_from(i).unwrap())
        .collect();
    let mut total_cost = 0;
    while let Some(pos) = find_next_region(&garden, &mut indexes) {
        total_cost += consume_cost_of_region(&mut garden, pos);
    }
    total_cost
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let garden = array2d::to_ndarray(&table::read_rows_chars(&input));
    let mut garden = garden.map(|x| Pixel::Val(*x));
    let mut indexes: Vec<Ix2s> = garden
        .indexed_iter()
        .map(|(i, _)| Ix2s::try_from(i).unwrap())
        .collect();
    let mut total_cost = 0;
    while let Some(pos) = find_next_region(&garden, &mut indexes) {
        total_cost += consume_discounted_cost_of_region(&mut garden, pos);
    }
    total_cost
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1370100);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 818286);
    }
}
