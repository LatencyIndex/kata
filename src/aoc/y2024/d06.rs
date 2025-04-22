use std::collections::HashSet;

use crate::lgl::data::table::{self, I2};

const INPUT: &str = "data/y2024/d06/input";

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Guard {
    pos: I2,
    dir: I2,
}

fn parse_guard_dir(c: &char) -> Option<I2> {
    match c {
        '^' => Some(I2(-1, 0)),
        '>' => Some(I2(0, 1)),
        'v' => Some(I2(1, 0)),
        '<' => Some(I2(0, -1)),
        _ => None,
    }
}

fn turn_right(dir: I2) -> I2 {
    match dir {
        I2(-1, 0) => I2(0, 1),
        I2(0, 1) => I2(1, 0),
        I2(1, 0) => I2(0, -1),
        I2(0, -1) => I2(-1, 0),
        _ => panic!("invalid direction"),
    }
}

fn tick_guard(obstructions: &[Vec<bool>], g: &Guard) -> Option<Guard> {
    let next_pos = g.pos + g.dir;
    table::get(obstructions, next_pos).map(|obstructed| {
        if *obstructed {
            Guard {
                pos: g.pos,
                dir: turn_right(g.dir),
            }
        } else {
            Guard {
                pos: next_pos,
                dir: g.dir,
            }
        }
    })
}

fn parse_input(input: &str) -> (Vec<Vec<bool>>, Guard) {
    let input = table::read_rows_chars(input);
    let obstructions: Vec<Vec<bool>> = input
        .iter()
        .map(|l| l.iter().map(|c| *c == '#').collect())
        .collect();
    let guards: Vec<Guard> = table::all_indexes(&input)
        .into_iter()
        .filter_map(|pos| {
            table::get(&input, pos).and_then(|c| parse_guard_dir(c).map(|dir| Guard { pos, dir }))
        })
        .collect();
    let guard = guards[0];
    (obstructions, guard)
}

/// Returns the path if it does not form a loop, and None otherwise.
fn get_open_path(obstructions: &[Vec<bool>], mut g: Guard) -> Option<HashSet<I2>> {
    let mut path: HashSet<Guard> = HashSet::new();
    loop {
        if path.insert(g) {
            // Value was newly inserted, i.e. we have not yet looped
            if let Some(g_next) = tick_guard(obstructions, &g) {
                // Next position is still on the map
                g = g_next;
            } else {
                // Next position is off the map, path is complete
                return Some(path.into_iter().map(|guard| guard.pos).collect());
            }
        } else {
            // Previous pos/dir visited, i.e. we are in a loop
            return None;
        }
    }
}

/// ```
/// use advent_of_code::aoc::y2024::d06::part1;
/// assert_eq!(part1(), 4559);
/// ```
pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (obstructions, guard) = parse_input(&input);
    get_open_path(&obstructions, guard).unwrap().len()
}

/// ```
/// use advent_of_code::aoc::y2024::d06::part2;
/// assert_eq!(part2(), 1604);
/// ```
pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (mut obstructions, guard) = parse_input(&input);
    let mut path = get_open_path(&obstructions, guard).unwrap();
    // New obstacle placement can be everywhere that is an intended next step,
    // and is on the map, and is not yet obstructed, and is not original guard position
    // This means every visited tile, except the first.
    path.remove(&guard.pos);

    let mut nb_cycles = 0;
    for r in path.into_iter() {
        // Place new obstruction
        *table::get_mut(&mut obstructions, r).unwrap() = true;
        let is_cycle = get_open_path(&obstructions, guard).is_none();
        nb_cycles += is_cycle as usize;
        // Remove new obstruction
        *table::get_mut(&mut obstructions, r).unwrap() = false;
    }
    nb_cycles
}
