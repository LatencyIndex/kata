use regex::Regex;

#[derive(Debug)]
enum Cmd {
    Mul(i64, i64),
    Do,
    Dont,
}

fn eval(cmds: impl Iterator<Item = Cmd>) -> i64 {
    let mut sum = 0;
    let mut enabled = 1;
    for cmd in cmds {
        match cmd {
            Cmd::Do => {
                enabled = 1;
            }
            Cmd::Dont => {
                enabled = 0;
            }
            Cmd::Mul(l, r) => {
                sum += enabled * l * r;
            }
        }
    }
    sum
}

const INPUT: &str = "data/y2024/d03/input";

/// ```
/// use advent_of_code::aoc::y2024::d03::part1;
/// assert_eq!(part1(), 161085926);
/// ```
pub fn part1() -> i64 {
    let hay = std::fs::read_to_string(INPUT).unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&hay)
        .map(|c| c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap())
        .sum()
}

/// ```
/// use advent_of_code::aoc::y2024::d03::part2;
/// assert_eq!(part2(), 82045421);
/// ```
pub fn part2() -> i64 {
    let hay = std::fs::read_to_string(INPUT).unwrap();

    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let muls = re_mul.captures_iter(&hay).map(|c| {
        (
            c.get(0).unwrap().start(),
            Cmd::Mul(c[1].parse().unwrap(), c[2].parse().unwrap()),
        )
    });
    let dos = re_do
        .captures_iter(&hay)
        .map(|c| (c.get(0).unwrap().start(), Cmd::Do));
    let donts = re_dont
        .captures_iter(&hay)
        .map(|c| (c.get(0).unwrap().start(), Cmd::Dont));
    let mut cmds: Vec<(usize, Cmd)> = muls.chain(dos).chain(donts).collect();
    cmds.sort_by_key(|(i, _)| *i);

    let cmds = cmds.into_iter().map(|(_, cmd)| cmd);
    eval(cmds)
}
