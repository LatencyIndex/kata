use crate::lgl::data::hashmap;
use petgraph::{
    algo::toposort,
    dot::Dot,
    graph::{DiGraph, NodeIndex},
    Direction,
};
use rand::Rng;
use regex::Regex;
use std::{collections::HashMap, fmt::Display, str::FromStr};

fn parse_val(s: &str) -> (String, u64) {
    let (var, val) = s.split_once(':').unwrap();
    (var.to_string(), val.trim().parse().unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn eval(&self, l: bool, r: bool) -> bool {
        match self {
            Self::And => l & r,
            Self::Or => l | r,
            Self::Xor => l ^ r,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::And => "&",
            Self::Or => "|",
            Self::Xor => "^",
        };
        s.fmt(f)
    }
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Op::And),
            "OR" => Ok(Op::Or),
            "XOR" => Ok(Op::Xor),
            x => Err(format!("cannot convert {x} to Op")),
        }
    }
}

#[derive(Debug)]
struct Expr {
    l: String,
    op: Op,
    r: String,
    out: String,
}

fn parse_expr(s: &str) -> Expr {
    let mut words = s.split_whitespace();
    let l = words.next().unwrap();
    let op = words.next().unwrap().parse().unwrap();
    let r = words.next().unwrap();
    let _arrow = words.next().unwrap();
    let out = words.next().unwrap();
    Expr {
        l: l.to_string(),
        op,
        r: r.to_string(),
        out: out.to_string(),
    }
}

fn parse_input(input: &str) -> (HashMap<String, u64>, Vec<Expr>) {
    let mut blocks = input.split("\n\n");
    let vals = blocks.next().unwrap();
    let exprs = blocks.next().unwrap();
    let vals = vals.lines().map(parse_val).collect();
    let exprs = exprs.lines().map(parse_expr).collect();
    (vals, exprs)
}

#[derive(Debug)]
struct Node {
    name: String,
    op: Option<Op>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.op {
            Some(op) => format!("{op} {}", self.name),
            None => self.name.to_string(),
        };
        s.fmt(f)
    }
}

struct Blank();

impl Display for Blank {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Result::Ok(())
    }
}

struct Circuit {
    graph: DiGraph<Node, Blank>,
    // Map from node name to node index
    nodes: HashMap<String, NodeIndex>,
    // Topological order of nodes
    topo: Vec<NodeIndex>,
}

impl Circuit {
    fn new(inputs: Vec<String>, mut exprs: Vec<Expr>) -> Self {
        let mut graph = DiGraph::new();
        let mut nodes = HashMap::new();
        for name in inputs {
            let i = graph.add_node(Node {
                name: name.clone(),
                op: None,
            });
            nodes.insert(name, i);
        }
        while !exprs.is_empty() {
            let (has_inputs, hasnt_inputs): (Vec<_>, Vec<_>) = exprs
                .into_iter()
                .partition(|expr| nodes.contains_key(&expr.l) && nodes.contains_key(&expr.r));
            // Add expressions that already have input nodes in graph
            for expr in has_inputs {
                let i = graph.add_node(Node {
                    name: expr.out.to_string(),
                    op: Some(expr.op),
                });
                nodes.insert(expr.out.to_string(), i);
                graph.add_edge(nodes[&expr.l], i, Blank());
                graph.add_edge(nodes[&expr.r], i, Blank());
            }
            // Keep only expressions that have not yet been added
            exprs = hasnt_inputs;
        }
        let topo = toposort(&graph, None).unwrap();
        Self { graph, nodes, topo }
    }
    fn eval(&self, inputs: &HashMap<String, bool>) -> HashMap<String, bool> {
        // Translate inputs from names to indices
        let mut vals: HashMap<NodeIndex, bool> =
            inputs.iter().map(|(k, v)| (self.nodes[k], *v)).collect();
        for i in self.topo.iter() {
            let node = self.graph.node_weight(*i).unwrap();
            if !vals.contains_key(i) {
                let mut ns = self.graph.neighbors_directed(*i, Direction::Incoming);
                let l = ns.next().unwrap();
                let r = ns.next().unwrap();
                vals.insert(*i, node.op.unwrap().eval(vals[&l], vals[&r]));
            }
        }
        let names = hashmap::invert(self.nodes.clone());
        vals.into_iter()
            .map(|(k, v)| (names[&k].clone(), v))
            .collect()
    }
    fn eval_from_inputs(&self, x: u64, y: u64) -> u64 {
        fn bit_i(u: u64, i: u32) -> bool {
            let b = (u >> i) & 1;
            b != 0
        }
        let mut inputs: HashMap<String, bool> = HashMap::new();
        let xre = Regex::new(r"^x(\d\d)$").unwrap();
        let yre = Regex::new(r"^y(\d\d)$").unwrap();
        let zre = Regex::new(r"^z(\d\d)$").unwrap();
        for node in self.graph.node_weights() {
            xre.captures(&node.name)
                .map(|cap| inputs.insert(node.name.clone(), bit_i(x, cap[1].parse().unwrap())));
            yre.captures(&node.name)
                .map(|cap| inputs.insert(node.name.clone(), bit_i(y, cap[1].parse().unwrap())));
        }
        let outputs = self.eval(&inputs);
        outputs
            .iter()
            .filter_map(|(k, v)| {
                zre.captures(k).map(|c| {
                    if *v {
                        let i: u32 = c[1].parse().unwrap();
                        2u64.pow(i)
                    } else {
                        0
                    }
                })
            })
            .sum()
    }
    fn rename_node(&mut self, src: &str, dst: &str) {
        println!("renaming {src} -> {dst}");
        let i = self.nodes.remove(src).unwrap();
        self.nodes.insert(dst.to_string(), i);
        self.graph.node_weight_mut(i).unwrap().name = dst.to_string();
    }
}

fn parse_name(hay: &str) -> Option<(char, u32)> {
    let re = Regex::new(r"^(.)(\d\d)$").unwrap();
    re.captures(hay)
        .map(|cap| (cap[1].chars().next().unwrap(), cap[2].parse().unwrap()))
}

// True if parents are the same level and specified characters, and the node itself has the given operation.
fn fits_half_pattern(
    graph: &DiGraph<Node, Blank>,
    i: NodeIndex,
    l_parent: char,
    r_parent: char,
    op: Op,
) -> Option<u32> {
    let node = graph.node_weight(i).unwrap();
    if node.op != Some(op) {
        return None;
    }
    // Node has already been processed.
    if parse_name(&node.name).is_some() {
        return None;
    }
    let mut inputs = graph.neighbors_directed(i, Direction::Incoming);
    let l = inputs
        .next()
        .and_then(|j| graph.node_weight(j).map(|n| &n.name))
        .and_then(|name| parse_name(name));
    let r = inputs
        .next()
        .and_then(|j| graph.node_weight(j).map(|n| &n.name))
        .and_then(|name| parse_name(name));
    if inputs.next().is_some() {
        return None;
    }
    match (l, r) {
        (Some((ln, li)), Some((rn, ri))) if ln == l_parent && rn == r_parent && li == ri => {
            Some(li)
        }
        _ => None,
    }
}

// Returns Some(parent_lvl) if pattern fits
fn fits_pattern(
    graph: &DiGraph<Node, Blank>,
    i: NodeIndex,
    parent_1: char,
    parent_2: char,
    op: Op,
) -> Option<u32> {
    fits_half_pattern(graph, i, parent_1, parent_2, op)
        .or(fits_half_pattern(graph, i, parent_2, parent_1, op))
}

const INPUT: &str = "data/y2024/d24/input";

pub fn part1() -> u64 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (vals, exprs) = parse_input(&input);
    let circuit = Circuit::new(vals.into_keys().map(|k| k.to_string()).collect(), exprs);
    // x and y values copied by hand from the test input.
    let x: u64 = 0b100001101011111001100100010100101111010101111;
    let y: u64 = 0b101001011110111101011000010101100010000011111;
    circuit.eval_from_inputs(x, y)
}

#[allow(unused)]
// Replace known-function nodes in the graph with mnemonics,
// and output the graph as a .svg for manual inspection.
fn circuit_mnemonics(circuit: &mut Circuit) {
    loop {
        let mut replacements = 0;
        for i in circuit.graph.node_indices() {
            // Direct carry d_i = x_i & y_i
            // Carry bit that ignores input carry bit
            if let Some(lvl) = fits_pattern(&circuit.graph, i, 'x', 'y', Op::And) {
                let old_name = circuit.graph.node_weight(i).unwrap().name.clone();
                let new_name = format!("a{:0>2}", lvl + 1);
                circuit.rename_node(&old_name, &new_name);
                replacements += 1;
            }
            // Partial sum p_i = x_i ^ y_i
            // Sum bit that ignores input carry bit.
            if let Some(lvl) = fits_pattern(&circuit.graph, i, 'x', 'y', Op::Xor) {
                let old_name = circuit.graph.node_weight(i).unwrap().name.clone();
                let new_name = format!("b{lvl:0>2}");
                circuit.rename_node(&old_name, &new_name);
                replacements += 1;
            }
            // b a and b d both apply to pattern c
            if let Some(lvl) = fits_pattern(&circuit.graph, i, 'b', 'a', Op::And) {
                let old_name = circuit.graph.node_weight(i).unwrap().name.clone();
                let new_name = format!("c{:0>2}", lvl + 1);
                circuit.rename_node(&old_name, &new_name);
                replacements += 1;
            }
            if let Some(lvl) = fits_pattern(&circuit.graph, i, 'b', 'd', Op::And) {
                let old_name = circuit.graph.node_weight(i).unwrap().name.clone();
                let new_name = format!("c{:0>2}", lvl + 1);
                circuit.rename_node(&old_name, &new_name);
                replacements += 1;
            }
            if let Some(lvl) = fits_pattern(&circuit.graph, i, 'a', 'c', Op::Or) {
                let old_name = circuit.graph.node_weight(i).unwrap().name.clone();
                let new_name = format!("d{:0>2}", lvl);
                circuit.rename_node(&old_name, &new_name);
                replacements += 1;
            }
        }
        if replacements == 0 {
            break;
        }
    }
    let dotfile = format!("{}", Dot::new(&circuit.graph));
    std::fs::write("graph.dot", dotfile).unwrap();
}

pub fn part2() -> String {
    let input = std::fs::read_to_string(INPUT).unwrap();
    // Swaps found by manually inspecting the circuit as a mnemonic .svg
    let swaps: HashMap<&str, &str> = [
        // swap 1: swap outputs of z08 and mvb
        ("sjd XOR mcr -> mvb", "sjd XOR mcr -> z08"),
        ("mcr AND sjd -> z08", "mcr AND sjd -> mvb"),
        // swap 2: swap outputs of a15 (rds) and b14 (jss)
        ("x14 AND y14 -> rds", "x14 AND y14 -> jss"),
        ("x14 XOR y14 -> jss", "x14 XOR y14 -> rds"),
        // swap 3: swap outputs of z18 and wss
        ("x18 AND y18 -> z18", "x18 AND y18 -> wss"),
        ("mfk XOR fmm -> wss", "mfk XOR fmm -> z18"),
        // swap 4: swap outputs of z23 and bmn
        ("fwj OR vsq -> z23", "fwj OR vsq -> bmn"),
        ("qmd XOR bpr -> bmn", "qmd XOR bpr -> z23"),
    ]
    .into_iter()
    .collect();
    let mut swap_outs = ["z08", "mvb", "jss", "rds", "wss", "z18", "bmn", "z23"];
    swap_outs.sort();

    let input: Vec<String> = input
        .lines()
        .map(|l| swaps.get(l).unwrap_or(&l).to_string())
        .collect();
    let input = input.join("\n");
    let (vals, exprs) = parse_input(&input);
    let circuit = Circuit::new(vals.into_keys().map(|k| k.to_string()).collect(), exprs);
    // Test the repaired circuit:
    // x and y each have 45 bits - x00 - x44, and y00 to y44
    let max: u64 = 1 << 45;
    let mut fails: u64 = 0;
    let mut rng = rand::thread_rng();
    // Increase iterations for more confidence
    for _ in 0..10 {
        let x: u64 = rng.gen_range(0..max);
        let y: u64 = rng.gen_range(0..max);
        let z = circuit.eval_from_inputs(x, y);
        let z_expected = x + y;
        let delta_bits = z ^ z_expected;
        fails |= delta_bits;
    }
    if fails == 0 {
        swap_outs.join(",")
    } else {
        "circuit is still broken".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 41324968993486);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), "bmn,jss,mvb,rds,wss,z08,z18,z23");
    }
}
