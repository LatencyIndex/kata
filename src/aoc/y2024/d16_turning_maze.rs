// Puzzle description: https://adventofcode.com/2024/day/16

use crate::lgl::data::{array2d, index::Ix2s};
use ndarray::Array2;
use petgraph::{algo::dijkstra::dijkstra, graphmap::DiGraphMap};
use std::collections::HashSet;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Coords {
    pos: Ix2s,
    dir: Ix2s,
}
type MazeGraph = DiGraphMap<Coords, i32>;

fn get_empty_tiles(maze: &Array2<char>) -> impl Iterator<Item = Ix2s> + '_ {
    maze.indexed_iter()
        .filter(|(_src, &val)| val != '#')
        .map(|(src, _val)| src.try_into().unwrap())
}

// All the edges connecting in the given direction.
fn get_straight_edges(
    maze: &Array2<char>,
    dir: Ix2s,
) -> impl Iterator<Item = (Coords, Coords, i32)> + '_ {
    get_empty_tiles(maze)
        .map(move |src| (src, src + dir))
        // Skip destination walls
        .filter(|(_src, dst)| array2d::get(maze, *dst).is_some_and(|&val| val != '#'))
        // Convert to coords and add edge cost
        .map(move |(src, dst)| (Coords { pos: src, dir }, Coords { pos: dst, dir }, 1))
}

// Clockwise cardinal directions
const CARDINAL_DIRS: [Ix2s; 4] = [
    Ix2s(-1, 0), // North
    Ix2s(0, 1),  // East
    Ix2s(1, 0),  // South
    Ix2s(0, -1), // West
];

fn get_turns(pos: Ix2s) -> impl Iterator<Item = (Coords, Coords, i32)> {
    CARDINAL_DIRS
        .iter()
        .zip(CARDINAL_DIRS.iter().cycle().skip(1))
        .flat_map(move |(&src_dir, &dst_dir)| {
            let src = Coords { pos, dir: src_dir };
            let dst = Coords { pos, dir: dst_dir };
            [(src, dst, 1000), (dst, src, 1000)]
        })
}

fn get_maze_graph(maze: &Array2<char>) -> MazeGraph {
    let straight_edges = CARDINAL_DIRS
        .iter()
        .flat_map(|&dir| get_straight_edges(maze, dir));
    let turn_edges = get_empty_tiles(maze).flat_map(get_turns);
    let edges = straight_edges.chain(turn_edges);
    MazeGraph::from_edges(edges)
}

#[allow(unused)]
fn show_path(maze: &Array2<char>, path: &[Ix2s]) -> String {
    let mut marked_maze = maze.clone();
    // Clear the maze
    for c in marked_maze.iter_mut().filter(|c| **c == '.') {
        *c = ' ';
    }
    // Mark the path
    for x in path {
        *array2d::get_mut(&mut marked_maze, *x).unwrap() = '.';
    }
    array2d::display(&marked_maze, "")
}

fn find_start(maze: &Array2<char>) -> Coords {
    Coords {
        pos: array2d::find_position(maze, &'S').unwrap(),
        dir: Ix2s(0, 1), // Start facing east
    }
}

fn find_goals(maze: &Array2<char>) -> [Coords; 4] {
    let pos = array2d::find_position(maze, &'E').unwrap();
    CARDINAL_DIRS.map(|dir| Coords { pos, dir })
}

const INPUT: &str = "data/y2024/d16/input";

pub fn part1() -> i32 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let maze_grid = array2d::from_chars(&input);
    let start = find_start(&maze_grid);
    let maze_graph = get_maze_graph(&maze_grid);
    let costs = dijkstra(&maze_graph, start, None, |edge| *edge.2);
    // For goal only position matters, not direction, so
    // find cost to the cheapest goal facing any direction
    *find_goals(&maze_grid)
        .iter()
        .filter_map(|g| costs.get(g))
        .min()
        .unwrap()
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let maze_grid = array2d::from_chars(&input);
    let start = find_start(&maze_grid);
    let maze_graph = get_maze_graph(&maze_grid);
    // How much it costs to reach a node from the start.
    let costs = dijkstra(&maze_graph, start, None, |edge| *edge.2);
    // For goal only position matters, not direction, so
    // find the cheapest goal to reach.
    let goal = *find_goals(&maze_grid)
        .iter()
        .filter(|g| costs.contains_key(g))
        .min_by_key(|g| costs[g])
        .unwrap();
    // Cost of the best path
    let min_cost = costs[&goal];

    let rev_graph = MazeGraph::from_edges(
        maze_graph
            .all_edges()
            .map(|(src, dst, val)| (dst, src, val)),
    );
    // How much it costs to reach each node from the end, in a reversed graph,
    // i.e. how much it costs to reach the end from that node.
    let rev_costs = dijkstra(&rev_graph, goal, None, |edge| *edge.2);
    let path_costs = costs
        .iter()
        .map(|(coords, cost)| (coords, cost + rev_costs[coords]));
    // Tile is part of best path if
    // path_cost(start, tile) + path_cost(tile, goal) = min_cost
    let best_tiles: HashSet<Ix2s> = path_costs
        .filter(|(_coords, cost)| *cost == min_cost)
        .map(|(coords, _cost)| coords.pos)
        .collect();
    best_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 88416);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 442);
    }
}
