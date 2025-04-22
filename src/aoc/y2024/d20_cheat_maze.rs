use crate::lgl::data::{array2d, graph, index::Ix2s};
use ndarray::Array2;
use petgraph::algo::dijkstra::dijkstra;

fn is_traversible(c: char) -> bool {
    c != '#'
}

fn get_empty_tiles(maze: &Array2<bool>) -> impl Iterator<Item = Ix2s> + '_ {
    maze.indexed_iter()
        .filter(|(_src, &val)| val)
        .map(|(src, _val)| src.try_into().unwrap())
}

fn find_cheats(maze: &Array2<bool>, src: Ix2s, len: isize) -> impl Iterator<Item = Ix2s> + '_ {
    let dsts = (-len..=len).flat_map(move |dx| {
        let rem = len - dx.abs();
        (-rem..=rem).map(move |dy| src + Ix2s(dx, dy))
    });
    dsts.filter(move |&dst| src != dst && array2d::get(maze, dst).copied().unwrap_or(false))
}

fn manhattan_norm(p: Ix2s) -> isize {
    p.0.abs() + p.1.abs()
}

fn count_cheats(
    maze_grid: &Array2<bool>,
    start: Ix2s,
    goal: Ix2s,
    // Maximum manhattan distance between cheat start and end.
    max_cheat_len: isize,
    min_save: i32,
) -> usize {
    let maze_graph = graph::from_maze_grid(maze_grid);
    let costs_from_start = dijkstra(&maze_graph, start, None, |_| 1);
    let costs_from_goal = dijkstra(&maze_graph, goal, None, |_| 1);
    let len_fair: i32 = graph::shortest_path(&maze_graph, &costs_from_start, goal)
        .len()
        .try_into()
        .unwrap();
    let mut saves = 0;
    for src in get_empty_tiles(maze_grid) {
        for dst in find_cheats(maze_grid, src, max_cheat_len) {
            let cheat_len: i32 = manhattan_norm(dst - src).try_into().unwrap();
            let len = costs_from_start[&src] + cheat_len + costs_from_goal[&dst];
            let save = len_fair - len;
            if min_save <= save {
                saves += 1;
            }
        }
    }
    saves
}

const INPUT: &str = "data/y2024/d20/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let maze_grid = array2d::from_chars(&input);
    let start = array2d::find_position(&maze_grid, &'S').unwrap();
    let goal = array2d::find_position(&maze_grid, &'E').unwrap();
    count_cheats(&maze_grid.map(|c| is_traversible(*c)), start, goal, 2, 100)
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let maze_grid = array2d::from_chars(&input);
    let start = array2d::find_position(&maze_grid, &'S').unwrap();
    let goal = array2d::find_position(&maze_grid, &'E').unwrap();
    count_cheats(&maze_grid.map(|c| is_traversible(*c)), start, goal, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1384);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1008542);
    }
}
