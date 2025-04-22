use crate::lgl::data::{array2d, graph, index::Ix2s};
use ndarray::Array2;
use petgraph::{algo::dijkstra::dijkstra, graphmap::UnGraphMap};

type MazeGraph = UnGraphMap<Ix2s, ()>;

/// Tiles marked 'true' are traversible, 'false' are not.
/// Tiles connect only in cardinal directions, not diagonally.
fn to_maze_graph(maze: &Array2<bool>) -> MazeGraph {
    let is_accessible = |p: Ix2s| -> bool { array2d::get(maze, p).copied().unwrap_or(false) };
    // Adjacent accessible tiles in the positive directions.
    // Because the graph is undirected, the negative directions will be
    // added by the other tile, avoiding duplicates.
    let pos_edges = |src: Ix2s| -> Vec<(Ix2s, Ix2s)> {
        let dsts = [src + Ix2s(0, 1), src + Ix2s(1, 0)];
        std::iter::repeat(src)
            .zip(dsts)
            .filter(|(src, dst)| is_accessible(*src) && is_accessible(*dst))
            .collect()
    };
    let edges = maze
        .indexed_iter()
        .flat_map(|(pos, _val)| pos_edges(pos.try_into().unwrap()));
    MazeGraph::from_edges(edges)
}

#[allow(unused)]
fn show_maze(maze: &Array2<bool>) -> Array2<char> {
    maze.map(|&v| if v { '.' } else { '#' })
}

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
    let maze_graph = to_maze_graph(&maze);
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
        let maze_graph = to_maze_graph(&maze);
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
