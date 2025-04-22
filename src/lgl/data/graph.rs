use crate::lgl::data::{array2d, index::Ix2s};
use ndarray::Array2;
use petgraph::{
    graphmap::UnGraphMap,
    visit::{GraphBase, IntoNeighborsDirected},
    Direction::Incoming,
};
use std::{collections::HashMap, hash::Hash};

/// A shortest path from a node with 0 cost (that defined the start) to the goal.
/// If the first element of the returned path is the start, and the last is the goal,
/// then a path was found.
/// 'costs' must be the output of Dijkstra's algorithm.
/// Warning: To avoid getting stuck in a cycle, it may not find the path if any edges are zero cost.
pub fn shortest_path<G, N>(graph: G, costs: &HashMap<N, i32>, goal: N) -> Vec<N>
where
    G: IntoNeighborsDirected + GraphBase<NodeId = N>,
    N: Eq + Hash + Clone,
{
    // Dijkstra's gives the length of the shortest path from start to every node.
    // To find the path, we start at the goal, and on every step,
    // move to the neighboring with the lowest cost, until reaching start.
    if let Some(goal_cost) = costs.get(&goal) {
        let mut path = vec![(goal, goal_cost)];
        loop {
            // Node we are trying to reach (we are traveling in reverse).
            let (dst_pos, dst_cost) = path.last().unwrap().clone();
            let in_neighbors = graph.neighbors_directed(dst_pos, Incoming);
            // Neighbor from which reaching dst is cheapest.
            let cheapest = in_neighbors
                .filter_map(|src_pos| costs.get(&src_pos).map(|src_cost| (src_pos, src_cost)))
                // Only move to cheapER neighbors, to avoid cycles.
                .filter(|(_src_pos, src_cost)| *src_cost < dst_cost)
                .min_by_key(|(_src_pos, src_cost)| *src_cost);
            if let Some(src) = cheapest {
                path.push(src);
            } else {
                // No next step is possible, or there is no cheaper neighbor,
                // meaning we could get stuck in a cycle. Instead we abort.
                break;
            }
        }
        path.into_iter().map(|(pos, _cost)| pos).rev().collect()
    } else {
        // Goal is unreachable
        Vec::new()
    }
}

/// Tiles marked 'true' are traversible, 'false' are not.
/// Tiles connect only in cardinal directions, not diagonally.
pub fn from_maze_grid(maze_grid: &Array2<bool>) -> UnGraphMap<Ix2s, ()> {
    let is_accessible = |p: Ix2s| -> bool { array2d::get(maze_grid, p).copied().unwrap_or(false) };
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
    let edges = maze_grid
        .indexed_iter()
        .flat_map(|(pos, _val)| pos_edges(pos.try_into().unwrap()));
    UnGraphMap::from_edges(edges)
}
