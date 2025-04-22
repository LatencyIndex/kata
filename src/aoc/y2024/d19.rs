/// Returns (available, requested) towel patterns, excluding whitespace or empty patterns.
fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (available, requested) = input.split_once("\n\n").unwrap();
    let available = available
        .split(", ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let requested = requested
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    (available, requested)
}

/// All available must be non-empty.
/// Too slow for real input, never returns.
#[allow(unused)]
fn is_possible(available: &[&str], requested: &str) -> bool {
    if requested.is_empty() {
        true
    } else {
        available
            .iter()
            .any(|a| requested.starts_with(a) && is_possible(available, &requested[a.len()..]))
    }
}

/// Nb. different ways the requested string can be created by concatenating available strings.
/// All available must be non-empty.
fn ways_to_match(available: &[&str], requested: &str) -> usize {
    let n = requested.len();
    // Nb. ways to match string up to index i.
    let mut nb_ways = Vec::from_iter(std::iter::repeat_n(0usize, n + 1));
    // String up to index 0 (i.e. the empty string) can be matched in 1 way.
    nb_ways[0] = 1;
    for i in 0..n {
        let remainder = &requested[i..];
        for a in available {
            if remainder.starts_with(a) {
                let j = i + a.len();
                // Found a way to match from up to i to up to j.
                // So ways to match up to j is increased by ways to match up to i.
                nb_ways[j] += nb_ways[i];
            }
        }
    }
    nb_ways[n]
}

const INPUT: &str = "data/y2024/d19/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (available, requested) = parse_input(&input);
    requested
        .iter()
        .filter(|r| 0 < ways_to_match(&available, r))
        .count()
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (available, requested) = parse_input(&input);
    requested.iter().map(|r| ways_to_match(&available, r)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 293);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 623924810770264);
    }
}
