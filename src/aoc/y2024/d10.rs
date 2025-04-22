use crate::lgl::data::{
    array2d::{self, Array2, Ix2},
    index::Ix2s,
    table,
};
use std::collections::HashSet;

fn parse_input(input: &str) -> Array2<i32> {
    array2d::to_ndarray(&table::read_rows_chars(input)).map(|c| *c as i32 - '0' as i32)
}

fn get_adjacent(map: &Array2<i32>, pos: Ix2, target_height: i32) -> Vec<Ix2> {
    let drs = vec![Ix2s(1, 0), Ix2s(-1, 0), Ix2s(0, 1), Ix2s(0, -1)];
    let pos: Ix2s = pos.try_into().unwrap();
    drs.into_iter()
        .map(|dr| pos + dr)
        .filter(|r| array2d::get(map, *r).is_some_and(|h| *h == target_height))
        .map(|r| r.try_into().unwrap())
        .collect()
}

// All adjacent tiles accessible from the given position.
fn get_children(map: &Array2<i32>, pos: Ix2) -> Vec<Ix2> {
    let target_height = map.get(pos).unwrap() + 1;
    get_adjacent(map, pos, target_height)
}

// All adjacent tiles with access to the given position.
fn get_parents(map: &Array2<i32>, pos: Ix2) -> Vec<Ix2> {
    let target_height = map.get(pos).unwrap() - 1;
    get_adjacent(map, pos, target_height)
}

// Expands visited tiles by one step.
// Moves unexplored tiles into explored, and fills unexplored with the newly added tiles.
fn add_adjacent_tiles(
    map: &Array2<i32>,
    // Tiles that have been visited, but their adjacent tiles have not yet been added.
    unexplored: &mut HashSet<Ix2>,
    // Tiles that have been visited, and their adjacent tiles added.
    explored: &mut HashSet<Ix2>,
) {
    // Adjacent tiles that have not yet been visited.
    let unvisited: HashSet<Ix2> = unexplored
        .iter()
        .flat_map(|pos| get_children(map, *pos))
        .filter(|pos| !unexplored.contains(pos) && !explored.contains(pos))
        .collect();
    explored.extend(unexplored.iter());
    *unexplored = unvisited;
}

// All the indexes with the given value
fn find_indexes(map: &Array2<i32>, val: i32) -> Vec<Ix2> {
    map.indexed_iter()
        .filter(|(_pos, h)| **h == val)
        .map(|((i, j), _h)| Ix2(i, j))
        .collect()
}

fn get_trailheads(map: &Array2<i32>) -> Vec<Ix2> {
    find_indexes(map, 0)
}

// Number of tiles with height 9 that are reachable from this position.
fn get_trail_score(map: &Array2<i32>, trailhead: Ix2) -> usize {
    let mut unexplored = HashSet::<Ix2>::new();
    let mut explored = HashSet::<Ix2>::new();
    unexplored.insert(trailhead);
    while !unexplored.is_empty() {
        add_adjacent_tiles(map, &mut unexplored, &mut explored);
    }
    explored.into_iter().filter(|pos| map[*pos] == 9).count()
}

struct TrailVertex {
    nb_paths: usize,
    nb_unexhausted_neighbors: usize,
}

// Number of different trails from each tile.
fn get_trail_ratings(map: &Array2<i32>) -> Array2<TrailVertex> {
    // The paths form a directed acyclic graph (but not a tree, paths can merge)

    // Number of paths from an arbitrary vertex is the sum of nbs of paths from its child vertexes,
    // or 1 if it has no child vertices, i.e. it is the end

    // Number of paths from a vertex is independent of where we start from,
    // meaning it is the same for all trailheads.

    // To find the trail rating:
    //      Assign end vertexes 1, and 0 to all others. Put end vertexes in the 'evaluated' set.
    //      For each evaluated vertex, add its value to its parents, and remove it from 'evaluated'
    //      When all of a vertexes children have added its value to it, add it to 'evaluated' set.

    // Vertexes for which the number of trails leading from them is known,
    // but have not yet propagated their values to their parents.
    let mut evaluated = Vec::<Ix2>::new();

    // Start by assigning end vertexes 1 path, and 0 to all others.
    let mut rating_map: Array2<TrailVertex> = map.map(|h| TrailVertex {
        nb_paths: (*h == 9) as usize,
        // Placeholder value, because index required to calculate it is unavaliable at this point.
        nb_unexhausted_neighbors: 0,
    });
    // Compute unexhausted neighbors, and update set of evaluated vertexes
    // (All vertexes with 0 children are evaluated, not just the end vertexes)
    rating_map.indexed_iter_mut().for_each(|((i, j), v)| {
        let pos = Ix2(i, j);
        v.nb_unexhausted_neighbors = get_children(map, pos).len();
        if v.nb_unexhausted_neighbors == 0 {
            evaluated.push(pos);
        }
    });

    while !evaluated.is_empty() {
        let mut newly_evaluated = Vec::<Ix2>::new();
        // For each evaluated vertex, add its value to its parents
        for &pos in evaluated.iter() {
            let nb_paths: usize = rating_map[pos].nb_paths;
            for parent_pos in get_parents(map, pos) {
                let parent = &mut rating_map[parent_pos];
                parent.nb_paths += nb_paths;
                parent.nb_unexhausted_neighbors -= 1;
                // If a vertex becomes fully evaluated, add it to 'evaluated' for next step
                if parent.nb_unexhausted_neighbors == 0 {
                    newly_evaluated.push(parent_pos);
                }
            }
        }
        evaluated = newly_evaluated;
    }
    rating_map
}

const INPUT: &str = "data/y2024/d10/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let map = parse_input(&input);
    get_trailheads(&map)
        .into_iter()
        .map(|pos| get_trail_score(&map, pos))
        .sum()
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let map = parse_input(&input);
    let trail_ratings = get_trail_ratings(&map);
    get_trailheads(&map)
        .into_iter()
        .map(|pos| trail_ratings[pos].nb_paths)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 841);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1875);
    }
}
