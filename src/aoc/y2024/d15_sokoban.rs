use crate::lgl::data::{array2d, index::Ix2s, table};
use ndarray::Array2;
use std::fmt::Display;

fn parse_move(c: char) -> Ix2s {
    // coords are y,x, from top-left
    match c {
        '<' => Ix2s(0, -1),
        '>' => Ix2s(0, 1),
        '^' => Ix2s(-1, 0),
        'v' => Ix2s(1, 0),
        _ => panic!("cannot parse {c} as a move"),
    }
}

fn parse_moves(s: &str) -> Vec<Ix2s> {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .map(parse_move)
        .collect()
}

fn parse_input(input: &str) -> (Warehouse, Vec<Ix2s>) {
    let mut inputs = input.split("\n\n");
    let map = inputs.next().unwrap();
    let moves = inputs.next().unwrap();
    (Warehouse::new(map), parse_moves(moves))
}

// Axis-aligned box on [min, max) interval
#[derive(Clone, Copy)]
struct Aab {
    min: Ix2s,
    max: Ix2s,
}

impl Aab {
    fn intersects(&self, other: &Aab) -> bool {
        let min0 = self.min.0.max(other.min.0);
        let min1 = self.min.1.max(other.min.1);
        let max0 = self.max.0.min(other.max.0);
        let max1 = self.max.1.min(other.max.1);
        min0 < max0 && min1 < max1
    }
    fn contains(&self, pos: &Ix2s) -> bool {
        self.min.0 <= pos.0 && pos.0 < self.max.0 && self.min.1 <= pos.1 && pos.1 < self.max.1
    }
    fn union(&self, other: &Aab) -> Aab {
        let min0 = self.min.0.min(other.min.0);
        let min1 = self.min.1.min(other.min.1);
        let max0 = self.max.0.max(other.max.0);
        let max1 = self.max.1.max(other.max.1);
        Aab {
            min: Ix2s(min0, min1),
            max: Ix2s(max0, max1),
        }
    }
    fn moved(&self, dir: Ix2s) -> Aab {
        Aab {
            min: self.min + dir,
            max: self.max + dir,
        }
    }
    fn widened(&self) -> Aab {
        Aab {
            min: Ix2s(self.min.0, self.min.1 * 2),
            max: Ix2s(self.max.0, self.max.1 * 2),
        }
    }
}

#[derive(Clone, Copy)]
struct Item {
    shape: Aab,
    can_move: bool,
}

impl Item {
    fn moved(&self, dir: Ix2s) -> Item {
        Item {
            shape: self.shape.moved(dir),
            ..*self
        }
    }
    fn gps(&self) -> isize {
        let Ix2s(y, x) = self.shape.min;
        100 * y + x
    }
    fn from_index_tile(r: (usize, usize), c: char) -> Option<Item> {
        let r: Ix2s = r.try_into().unwrap();
        let shape = Aab {
            min: r,
            max: r + Ix2s(1, 1),
        };
        match c {
            // Wall
            '#' => Some(Item {
                shape,
                can_move: false,
            }),
            // Box
            'O' => Some(Item {
                shape,
                can_move: true,
            }),
            _ => None,
        }
    }
}

struct Warehouse {
    items: Vec<Item>,
    robot: Ix2s,
}

impl Warehouse {
    fn new(s: &str) -> Warehouse {
        let map = array2d::to_ndarray(&table::read_rows_chars(s));
        let robot = map
            .indexed_iter()
            .find(|(_pos, tile)| **tile == '@')
            .map(|(pos, _tile)| pos)
            .unwrap()
            .try_into()
            .unwrap();
        let items = map
            .indexed_iter()
            .filter_map(|(pos, tile)| Item::from_index_tile(pos, *tile))
            .collect();
        Warehouse { items, robot }
    }
    fn widen(&self) -> Warehouse {
        let items = self
            .items
            .iter()
            .map(|x| Item {
                shape: x.shape.widened(),
                ..*x
            })
            .collect();
        let robot = Ix2s(self.robot.0, 2 * self.robot.1);
        Warehouse { items, robot }
    }
    fn move_robot(&mut self, dir: Ix2s) {
        let robot_next = self.robot + dir;

        // Initialize pushed vecs with items directly pushed by the robot
        let pushed_by_robot = |x: &&Item| -> bool { x.shape.contains(&robot_next) };
        let (
            // Items that have been pushed, but items they hit have not yet been found.
            mut newly_pushed,
            // Items that have not yet been pushed.
            mut not_pushed,
        ): (Vec<Item>, Vec<Item>) = self.items.iter().partition(pushed_by_robot);
        newly_pushed = newly_pushed.into_iter().map(|x| x.moved(dir)).collect();
        // Items that have been pushed, and all the items they hit have been found.
        let mut pushed: Vec<Item> = Vec::new();

        // Chain reaction of items pushing other items.
        while !newly_pushed.is_empty() {
            if newly_pushed.iter().any(|x| !x.can_move) {
                // To execute the move, an immovable object would have to be pushed.
                return;
            }
            // Items hit/missed by newly pushed.
            let got_hit = |x: &Item| -> bool {
                newly_pushed
                    .iter()
                    .any(|pusher| pusher.shape.intersects(&x.shape))
            };
            let (hits, misses): (Vec<Item>, Vec<Item>) = not_pushed.into_iter().partition(got_hit);
            // Not pushed has now been divided between hits & misses.
            not_pushed = misses;
            // Newly pushed have been processed.
            pushed.extend(newly_pushed);
            // Now hits have to be processed.
            newly_pushed = hits.into_iter().map(|x| x.moved(dir)).collect();
        }
        // All items have been either pushed or not pushed, and no immovables were encountered.
        self.items = not_pushed.into_iter().chain(pushed).collect();
        self.robot = robot_next;
    }
    fn gps_sum(&self) -> isize {
        self.items
            .iter()
            .filter(|&x| x.can_move)
            .map(|x| x.gps())
            .sum()
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let zero = Aab {
            min: Ix2s(0, 0),
            max: Ix2s(0, 0),
        };
        let size: Aab = self.items.iter().fold(zero, |acc, x| acc.union(&x.shape));
        let n0: usize = (size.max.0 - size.min.0).try_into().unwrap();
        let n1: usize = (size.max.1 - size.min.1).try_into().unwrap();
        let mut arr = Array2::from_elem((n0, n1), '.');
        for item in self.items.iter() {
            let shape = &item.shape;
            let c = if item.can_move { 'O' } else { '#' };
            for i0 in shape.min.0..shape.max.0 {
                for i1 in shape.min.1..shape.max.1 {
                    let pos = Ix2s(i0, i1);
                    *array2d::get_mut(&mut arr, pos).unwrap() = c;
                }
            }
        }
        *array2d::get_mut(&mut arr, self.robot).unwrap() = '@';
        write!(f, "{}", array2d::display_chars(&arr))
    }
}

const INPUT: &str = "data/y2024/d15/input";

pub fn part1() -> isize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (mut warehouse, moves) = parse_input(&input);
    for m in moves {
        warehouse.move_robot(m);
    }
    warehouse.gps_sum()
}

pub fn part2() -> isize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (warehouse, moves) = parse_input(&input);
    let mut warehouse = warehouse.widen();
    for m in moves {
        warehouse.move_robot(m);
    }
    warehouse.gps_sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1515788);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1516544);
    }
}
