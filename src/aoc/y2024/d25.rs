use crate::lgl::data::array2d;

type Pins = Vec<usize>;

// True for lock, false for key
fn parse_tumbler(s: &str) -> (bool, Pins) {
    let arr = array2d::from_chars(s);
    let is_lock = arr.row(0).iter().all(|x| *x == '#');
    let pins: Vec<usize> = arr
        .columns()
        .into_iter()
        .map(|col| col.iter().filter(|x| **x == '#').count() - 1)
        .collect();
    (is_lock, pins)
}

// Returns (locks, keys)
fn parse_input(input: &str) -> (Vec<Pins>, Vec<Pins>) {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .map(parse_tumbler)
        .partition(|(is_lock, _pins)| *is_lock);
    (
        locks.into_iter().map(|(_, pins)| pins).collect(),
        keys.into_iter().map(|(_, pins)| pins).collect(),
    )
}

fn fit(key: &Pins, lock: &Pins) -> bool {
    key.iter().zip(lock).all(|(k, l)| k + l < 6)
}

const INPUT: &str = "data/y2024/d25/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let (locks, keys) = parse_input(&input);
    locks
        .iter()
        .map(|lock| keys.iter().filter(|key| fit(lock, key)).count())
        .sum()
}

pub fn part2() -> usize {
    // let input = std::fs::read_to_string(INPUT).unwrap();
    2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 3136);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2);
    }
}
