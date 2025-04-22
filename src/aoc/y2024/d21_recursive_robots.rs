use crate::lgl::data::index::Ix2s;
use petgraph::{algo::dijkstra, graphmap::DiGraphMap};
use std::{collections::HashMap, hash::Hash};

fn get_numpad() -> Keypad {
    Keypad::from_iter([
        (Ix2s(0, 0), '7'),
        (Ix2s(1, 0), '8'),
        (Ix2s(2, 0), '9'),
        (Ix2s(0, 1), '4'),
        (Ix2s(1, 1), '5'),
        (Ix2s(2, 1), '6'),
        (Ix2s(0, 2), '1'),
        (Ix2s(1, 2), '2'),
        (Ix2s(2, 2), '3'),
        // Invalid coords are omitted
        // (Ix2s(0, 3), ' '),
        (Ix2s(1, 3), '0'),
        (Ix2s(2, 3), 'A'),
    ])
}

fn get_arrowpad() -> Keypad {
    Keypad::from_iter([
        // Invalid coords are omitted
        // (Ix2s(0, 0), ' '),
        (Ix2s(1, 0), '^'),
        (Ix2s(2, 0), 'A'),
        (Ix2s(0, 1), '<'),
        (Ix2s(1, 1), 'v'),
        (Ix2s(2, 1), '>'),
    ])
}

// Buttons that crash the robot are omitted from the hashmap.
// I.e. if keypad.get(robot_pos).is_none(), then the robot crashed.
type Keypad = HashMap<Ix2s, char>;

fn invert_hashmap<K, V: Eq + Hash>(hmap: HashMap<K, V>) -> HashMap<V, K> {
    hmap.into_iter().map(|(k, v)| (v, k)).collect()
}

// Next state of the robot arm if it receives cmd.
fn eval_cmd(keypad: &Keypad, arm: char, cmd: char) -> Option<char> {
    if cmd == 'A' {
        // Button press - arm does not move
        Some(arm)
    } else {
        let delta = match cmd {
            // xy 0,0 is top-left corner
            '>' => Ix2s(1, 0),
            '<' => Ix2s(-1, 0),
            '^' => Ix2s(0, -1),
            'v' => Ix2s(0, 1),
            c => panic!("invalid arrowpad button pressed: {c:?}"),
        };
        let old_pos = invert_hashmap(keypad.clone())[&arm];
        let new_pos = old_pos + delta;
        keypad.get(&new_pos).copied()
    }
}

// Let slave arm be on button 'src', having just clicked it.
// Cost to click 'dst' depends only on 'src', because master starts and ends hovering over 'A'
// However, as the arm moves, the cost of each edge depends on the state of its master,
// because the arm did not click, therefore master will not always be in the same state.

// Map of costmatrix[src][dst] is cost to click dst key if starting from src key.
type CostMatrix = HashMap<char, HashMap<char, i64>>;
// Cost to click any key on master arrowpad, regardless of starting point, is 1.
fn get_master_matrix(keypad: &Keypad) -> CostMatrix {
    HashMap::from_iter(keypad.values().map(|&src| {
        (
            src,
            HashMap::from_iter(keypad.values().map(|&dst| (dst, 1))),
        )
    }))
}

// Given cost matrix for master arm, how to generate cost matrix for slave arm?
// Let node be (slave_pos, master_pos)
type Node = (char, char);
// Cost of movement:
// Every button that master can click (except A) defines next (slave_pos, master_pos)
// Cost of this edge is defined by cost of (master_prev, master_next)
// This defines neighbor nodes, which defines a graph.
fn get_out_edges<'a>(
    slave_keypad: &'a Keypad,
    master_matrix: &'a CostMatrix,
    src_node: Node,
) -> impl Iterator<Item = (Node, Node, i64)> + use<'a> {
    let (slave_src, master_src) = src_node;
    // Every command and its cost, given the current state of master
    master_matrix[&master_src]
        .iter()
        // Find how the commands affect the slave
        .filter_map(
            move |(&master_dst /*the command applied to slave*/, &cost)| {
                eval_cmd(slave_keypad, slave_src, master_dst)
                    .map(|slave_dst| ((slave_dst, master_dst), cost))
            },
        )
        // This defines all out-neighbors for current (slave,master) state.
        .map(move |(dst_node, cost)| (src_node, dst_node, cost))
}

// All the edges that define how a master/slave set of robot arms behave
fn get_edges<'a>(
    slave_keypad: &'a Keypad,
    master_matrix: &'a CostMatrix,
) -> impl Iterator<Item = (Node, Node, i64)> + use<'a> {
    // Every (slave_src, master_src) combination
    let src_nodes = slave_keypad.values().flat_map(move |&slave_src| {
        master_matrix
            .keys()
            .map(move |&master_src| (slave_src, master_src))
    });
    src_nodes.flat_map(|src_node| get_out_edges(slave_keypad, master_matrix, src_node))
}

// Form a graph from these edges. Cost matrix is defined by shortest path from pos to click.
fn get_slave_matrix(slave_keypad: &Keypad, master_matrix: &CostMatrix) -> CostMatrix {
    let graph: DiGraphMap<Node, i64> =
        DiGraphMap::from_iter(get_edges(slave_keypad, master_matrix));
    // Because we always move from having just clicked src button,
    // to clicking dst button, master arm will be over 'A' button in both cases,
    // as that is the only button that instructs slave to click.
    HashMap::from_iter(slave_keypad.values().map(|&slave_src| {
        let costs = dijkstra(&graph, (slave_src, 'A'), None, |edge| *edge.2);
        (
            slave_src,
            HashMap::from_iter(
                slave_keypad
                    .values()
                    // If src == dst, dijkstra yields 0 cost, but.. TODO
                    .map(|&slave_dst| (slave_dst, costs[&(slave_dst, 'A')].max(1))),
            ),
        )
    }))
}

fn get_cost(cost_matrix: &CostMatrix, buttons: &str) -> i64 {
    let mut cost = 0;
    // Start every sequence over the 'A' button.
    let prev_buttons = std::iter::once('A').chain(buttons.chars());
    for (src, dst) in prev_buttons.zip(buttons.chars()) {
        // Cost matrix only yields the cost to reach a button.
        // To click it, it costs 1 more.
        // TODO: Am I considering this +1 everywhere I need to?
        // But using 0 instead of 1 yields correct result...
        cost += cost_matrix[&src][&dst] + 0;
    }
    cost
}

fn get_num(code: &str) -> i64 {
    let n: String = code.chars().take_while(|c| c.is_ascii_digit()).collect();
    n.parse().unwrap()
}

const INPUT: &str = "data/y2024/d21/input";

pub fn part1() -> i64 {
    let codes = std::fs::read_to_string(INPUT).unwrap();
    let nb_robot_arrowpads = 2;
    let mut cost_matrix = get_master_matrix(&get_arrowpad());
    for _ in 0..nb_robot_arrowpads {
        cost_matrix = get_slave_matrix(&get_arrowpad(), &cost_matrix);
    }
    cost_matrix = get_slave_matrix(&get_numpad(), &cost_matrix);

    let mut complexity = 0;
    for code in codes.lines() {
        let cost = get_cost(&cost_matrix, code);
        let num = get_num(code);
        complexity += cost * num;
    }
    complexity
}

pub fn part2() -> i64 {
    let codes = std::fs::read_to_string(INPUT).unwrap();
    let nb_robot_arrowpads = 25;
    let mut cost_matrix = get_master_matrix(&get_arrowpad());
    for _ in 0..nb_robot_arrowpads {
        cost_matrix = get_slave_matrix(&get_arrowpad(), &cost_matrix);
    }
    cost_matrix = get_slave_matrix(&get_numpad(), &cost_matrix);

    let mut complexity = 0;
    for code in codes.lines() {
        let cost = get_cost(&cost_matrix, code);
        let num = get_num(code);
        complexity += cost * num;
    }
    complexity
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 237342);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 294585598101704);
    }
}
