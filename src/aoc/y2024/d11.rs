use std::collections::HashMap;

fn nb_digits(i: u64) -> u32 {
    if i == 0 {
        1
    } else {
        i.ilog10() + 1
    }
}

fn split(i: u64, rdigits: u32) -> Vec<u64> {
    let n = 10u64.pow(rdigits);
    vec![i / n, i % n]
}

fn get_children(i: u64) -> Vec<u64> {
    if i == 0 {
        vec![1]
    } else {
        let nb_digits = nb_digits(i);
        if nb_digits % 2 == 0 {
            split(i, nb_digits / 2)
        } else {
            vec![2024 * i]
        }
    }
}

fn apply_rules(stones: &[u64]) -> Vec<u64> {
    stones.iter().flat_map(|i| get_children(*i)).collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Stone {
    number: u64,
    blinks_left: usize,
}

impl Stone {
    fn children(&self) -> Vec<Stone> {
        if 0 < self.blinks_left {
            get_children(self.number)
                .into_iter()
                .map(|number| Stone {
                    number,
                    blinks_left: self.blinks_left - 1,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    fn nb_children(&self) -> usize {
        // Optimization of self.children().len()
        if self.blinks_left == 0 {
            0
        } else {
            2 - (nb_digits(self.number) % 2) as usize
        }
    }
}

struct Cache {
    cached: HashMap<Stone, usize>,
}

impl Cache {
    fn insert(&mut self, key: Stone, val: usize) {
        self.cached.insert(key, val);
    }
    fn get(&self, key: &Stone) -> Option<usize> {
        if key.blinks_left == 0 {
            Some(1)
        } else if key.blinks_left == 1 {
            Some(key.nb_children())
        } else {
            self.cached.get(key).copied()
        }
    }
}

#[allow(unused)]
fn nb_descendants_recursive(stone: &Stone, cache: &mut Cache) -> usize {
    if let Some(n) = cache.get(stone) {
        n
    } else {
        let n = stone
            .children()
            .iter()
            .map(|stone| nb_descendants_recursive(stone, cache))
            .sum();
        cache.insert(stone.clone(), n);
        n
    }
}

#[derive(Debug)]
struct Entry {
    cmd: Stone,
    // Stack line where this cmd's args begin
    arg: Option<usize>,
    val: Option<usize>,
}

struct CallStack {
    stack: Vec<Entry>,
    ptr: usize,
    cache: Cache,
}

impl CallStack {
    fn new(cmd: Stone) -> CallStack {
        CallStack {
            stack: vec![Entry {
                cmd,
                arg: None,
                val: None,
            }],
            ptr: 0,
            cache: Cache {
                cached: HashMap::new(),
            },
        }
    }
    fn value(&self) -> Option<usize> {
        match self.stack.first() {
            Some(entry) => entry.val,
            None => None,
        }
    }
    fn step(&mut self) {
        let entry = &self.stack[self.ptr];

        if entry.val.is_some() {
            // Cmd has already been evaluated, so go to parent cmd.
            self.ptr = self.ptr.saturating_sub(1);
        } else if let Some(arg) = entry.arg {
            // Required args are on the stack, so cmd can be evaluated. Includes the 0 args case.
            // Evaluate cmd
            // If a stone splits in 2, that counts as having 2 children.
            // But if it does not split, that counts as 0 children, so we add 1 for the stone itself.
            let args = &self.stack[arg..];
            let nb_children: usize = args.iter().map(|e| e.val.unwrap()).sum();
            let val = nb_children.max(1);
            // Update the stack entry
            let entry = &mut self.stack[self.ptr];
            entry.val = Some(val);
            // Update cache
            self.cache.insert(entry.cmd.clone(), val);
            // Drop used-up args/children
            self.stack.truncate(arg);
        } else if let Some(cached) = self.cache.get(&entry.cmd) {
            // Update stack entry with cached value
            let entry = &mut self.stack[self.ptr];
            entry.val = Some(cached);
        } else {
            let children = entry.cmd.children();
            // Set arg ptr to where children will be placed
            self.stack[self.ptr].arg = Some(self.stack.len());
            // Push unevaluated args/children to the stack
            self.stack.extend(children.into_iter().map(|cmd| Entry {
                cmd,
                arg: None,
                val: None,
            }));
            // Move stack ptr to top
            self.ptr = self.stack.len() - 1;
        }
    }
    fn eval(&mut self) -> usize {
        loop {
            if let Some(val) = self.value() {
                return val;
            } else {
                self.step();
            }
        }
    }
}

fn parse_stones(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

const INPUT: &str = "data/y2024/d11/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let mut stones = parse_stones(&input);
    for _ in 0..25 {
        stones = apply_rules(&stones);
    }
    stones.len()
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let stones = parse_stones(&input);
    let blinks_left = 75;
    let stones = stones.into_iter().map(|number| Stone {
        number,
        blinks_left,
    });
    stones.map(|s| CallStack::new(s).eval()).sum()
    // let mut cache = Cache {
    //     cached: HashMap::new(),
    // };
    // stones
    //     .map(|s| nb_descendants_recursive(&s, &mut cache))
    //     .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 202019);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 239321955280205);
    }
}
