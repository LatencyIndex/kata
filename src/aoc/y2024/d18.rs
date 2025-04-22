use crate::lgl::data::{array2d, index::Ix2s, table};
use ndarray::Array2;
use petgraph::{algo::dijkstra::dijkstra, graphmap::UnGraphMap, Direction::Incoming};
use std::collections::HashMap;

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

fn shortest_path(
    maze: &MazeGraph,
    costs: &HashMap<Ix2s, i32>,
    start: Ix2s,
    goal: Ix2s,
) -> Vec<Ix2s> {
    let mut path = vec![goal];
    while *path.last().unwrap() != start {
        let adjacents = maze.neighbors_directed(*path.last().unwrap(), Incoming);
        let cheapest = adjacents
            .filter_map(|pos| costs.get(&pos).map(|cost| (pos, cost)))
            .min_by_key(|(_pos, cost)| *cost);
        if let Some((pos, _cost)) = cheapest {
            path.push(pos);
        } else {
            break;
        }
    }
    path.reverse();
    path
}

const INPUT: &str = "data/y2024/d18/input";

pub fn part1() -> usize {
    let falling = std::fs::read_to_string(INPUT).unwrap();
    let falling: Array2<isize> =
        array2d::to_ndarray(&table::read_rows(&falling, ",")).map(|x| x.parse().unwrap());
    let falling: Vec<Ix2s> = falling
        .rows()
        .into_iter()
        // Ix2 & Ix2s use Y,X coords, but input is in X,Y
        .map(|row| Ix2s(row[1], row[0]))
        .collect();
    let shape = (71, 71);
    let start = Ix2s(0, 0);
    let goal = Ix2s(70, 70);
    let mut maze = Array2::from_elem(shape, true);
    for b in falling.into_iter().take(1024) {
        *array2d::get_mut(&mut maze, b).unwrap() = false;
    }
    let maze_graph = to_maze_graph(&maze);
    let costs = dijkstra(&maze_graph, start, None, |_| 1);
    let path = shortest_path(&maze_graph, &costs, start, goal);
    // To get nb. of steps, subtract 1 because start position,
    // included in the path, takes 0 steps.
    path.len() - 1
}

pub fn part2() -> String {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let input: Array2<isize> =
        array2d::to_ndarray(&table::read_rows(&input, ",")).map(|x| x.parse().unwrap());
    let falling: Vec<Ix2s> = input
        .rows()
        .into_iter()
        // Ix2 & Ix2s use Y,X coords, but input is in X,Y
        .map(|row| Ix2s(row[1], row[0]))
        .collect();
    let shape = (71, 71);
    let start = Ix2s(0, 0);
    let goal = Ix2s(70, 70);
    let mut maze = Array2::from_elem(shape, true);
    // Could use bisection search if we were feeling fancy, but it's fast enough.
    for b in falling.into_iter() {
        *array2d::get_mut(&mut maze, b).unwrap() = false;
        let maze_graph = to_maze_graph(&maze);
        let costs = dijkstra(&maze_graph, start, None, |_| 1);
        if !costs.contains_key(&goal) {
            return format!("{},{}", b.1, b.0);
        }
    }
    "Not found".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 284);
    }
    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(), "51,50");
    }
}
