// Puzzle description: https://adventofcode.com/2024/day/18

use crate::lgl::data::{array2d, graph, index::Ix2s};
use ndarray::Array2;
use petgraph::algo::dijkstra::dijkstra;

// Find largest i on [lo, hi) interval for which p(i) == false.
// Assumes that p monotonically increases under the false < true ordering,
// and that p(lo) == false and p(hi) == true.
fn bisect_predicate<P>(p: P, mut lo: usize, mut hi: usize) -> usize
where
    P: Fn(usize) -> bool,
{
    // [..lo] is false
    // [hi..] is true
    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        if p(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    lo
}

// Read rows of X,Y integers.
fn parse_input(input: &str) -> Vec<Ix2s> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            // Ix2s uses Y,X coords, but input is in X,Y
            Ix2s(y.parse().unwrap(), x.parse().unwrap())
        })
        .collect()
}

const INPUT: &str = "data/y2024/d18/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let falling = parse_input(&input);
    let shape = (71, 71);
    let start = Ix2s(0, 0);
    let goal = Ix2s(70, 70);
    let mut maze = Array2::from_elem(shape, true);
    for b in falling.into_iter().take(1024) {
        *array2d::get_mut(&mut maze, b).unwrap() = false;
    }
    let maze_graph = graph::from_maze_grid(&maze);
    let costs = dijkstra(&maze_graph, start, Some(goal), |_| 1);
    let path = graph::shortest_path(&maze_graph, &costs, goal);
    // To get nb. of steps, subtract 1 because start position,
    // included in the path, takes 0 steps.
    path.len() - 1
}

pub fn part2() -> String {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let falling = parse_input(&input);
    let shape = (71, 71);
    let start = Ix2s(0, 0);
    let goal = Ix2s(70, 70);
    let maze = Array2::from_elem(shape, true);
    let goal_is_unreachable = |i: usize| -> bool {
        let mut maze = maze.clone();
        for b in falling.iter().take(i) {
            *array2d::get_mut(&mut maze, *b).unwrap() = false;
        }
        let maze_graph = graph::from_maze_grid(&maze);
        let costs = dijkstra(&maze_graph, start, Some(goal), |_| 1);
        !costs.contains_key(&goal)
    };
    // How many tiles can be blocked by the falling bytes, without blocking the goal.
    let nb_reachable = bisect_predicate(goal_is_unreachable, 0, falling.len());
    let blocking_byte = falling[nb_reachable];
    format!("{},{}", blocking_byte.1, blocking_byte.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 284);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), "51,50");
    }
    #[test]
    fn test_bisect() {
        let p = |i: usize| -> bool { 5 < i };
        let i = bisect_predicate(p, 0, 10);
        assert_eq!(i, 5);
    }
}
