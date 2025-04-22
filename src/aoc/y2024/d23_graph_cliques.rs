// Puzzle description: https://adventofcode.com/2024/day/23

use std::collections::{BTreeMap, BTreeSet};

type Network<'a> = BTreeMap<&'a str, BTreeSet<&'a str>>;

// All the cycles of 3 nodes that node a is part of.
fn triangles<'a>(net: &'a Network, a: &'a str) -> Vec<[&'a str; 3]> {
    let mut triangles = Vec::new();
    let neighbors_a = &net[a];
    for b in neighbors_a {
        let neighbors_b = &net[b];
        let mutuals = neighbors_a.intersection(neighbors_b);
        triangles.extend(mutuals.map(|c| [a, b, c]));
    }
    for t in triangles.iter_mut() {
        t.sort();
    }
    triangles
}

fn all_triangles<'a>(net: &'a Network) -> BTreeSet<[&'a str; 3]> {
    net.keys().flat_map(|a| triangles(net, a)).collect()
}

fn parse_network(input: &str) -> Network {
    let edges = input.lines().map(|l| l.split_once('-').unwrap());
    let mut net = Network::new();
    for (a, b) in edges {
        net.entry(a).or_default().insert(b);
        net.entry(b).or_default().insert(a);
    }
    net
}

const INPUT: &str = "data/y2024/d23/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let net = parse_network(&input);
    all_triangles(&net)
        .iter()
        .filter(|t| t.iter().any(|x| x.starts_with("t")))
        .count()
}

pub fn part2() -> String {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let net = parse_network(&input);
    // Sets of nodes and their neighbors
    let mut nsets: Vec<BTreeSet<&str>> = net
        .into_iter()
        .map(|(k, v)| std::iter::once(k).chain(v).collect())
        .collect();
    nsets.sort();
    // Every node has the same nb. of neighbors (13).
    // The biggest clique possible is thus 14, but empirically it is at least 1 smaller.
    // If a node with n neighbors is part of a clique of size k, then there are
    // binomial(n,k) possible combinations of its neighbors that form the clique.
    // Thus if k is very close to n, there are not many combinations (n for k+1 = n).
    // Finding the largest clique is NP-complete in general, so we assume that k is close to n,
    // and start the search at k == n, and decrement k if we don't find anything.
    // It turns out the largest clique is of size n-1.
    let mut max_clique = BTreeSet::new();
    for set in nsets.iter() {
        for i in 0..set.len() {
            // Create subset by removing 1 element
            let subset: BTreeSet<&str> = set
                .iter()
                .enumerate()
                .filter(|&(j, _x)| i != j)
                .map(|(_j, x)| *x)
                .collect();
            // Count how many times it appears in all nsets.
            let n = nsets.iter().filter(|s| subset.is_subset(s)).count();
            // If it appears at least as many times as its length,
            // then all of its members are part of a clique.
            if subset.len() <= n {
                max_clique = subset;
            }
        }
    }
    let max_clique: Vec<_> = max_clique.into_iter().collect();
    max_clique.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1467);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), "di,gs,jw,kz,md,nc,qp,rp,sa,ss,uk,xk,yn");
    }
}
