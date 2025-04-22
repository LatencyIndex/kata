use std::ops::{Add, Mul};

use crate::lgl::data::table;

#[derive(Clone, Copy)]
struct Vec2<T>(T, T);

impl<T: Add> Add for Vec2<T> {
    type Output = Vec2<<T as Add>::Output>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Mul + Copy> Mul<T> for Vec2<T> {
    type Output = Vec2<<T as Mul<T>>::Output>;
    fn mul(self, rhs: T) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

type I2 = Vec2<i32>;

fn table_get<T>(table: &[Vec<T>], p: I2) -> Option<&T> {
    let i: usize = p.0.try_into().ok()?;
    let j: usize = p.1.try_into().ok()?;
    table.get(i).and_then(|v| v.get(j))
}

fn is_needle(needle: &str, table: &[Vec<char>], origin: I2, dir: I2) -> bool {
    let coords = (0i32..).map(|k| origin + dir * k);
    let word = coords
        .map_while(|p| table_get(table, p).copied())
        .take(needle.len());
    word.eq(needle.chars())
}

fn get_origins(table: &[Vec<char>]) -> Vec<Vec2<i32>> {
    let mut origins = Vec::new();
    for (i, line) in table.iter().enumerate() {
        for j in 0..line.len() {
            origins.push(Vec2(i as i32, j as i32));
        }
    }
    origins
}

/// Counts how many times the word appears in the table.
/// Occurences can be vertical, horizontal, or diagonal, forwards and backwards.
fn count_word(needle: &str, hay: &str) -> usize {
    let dirs: Vec<Vec2<i32>> = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
    .into_iter()
    .map(|(i, j)| Vec2(i, j))
    .collect();
    let hay: Vec<Vec<char>> = table::read_rows_chars(hay);
    let mut hits = 0;
    for origin in get_origins(&hay) {
        for dir in dirs.iter() {
            let hit = is_needle(needle, &hay, origin, *dir);
            hits += hit as usize;
        }
    }
    hits
}

/// Whether the center represents to diagonal 'MAS' words crossing, e.g.
/// M . S     S . S
/// . A . or  . A .
/// M . S     M . M
fn is_x_mas(table: &[Vec<char>], center: I2) -> bool {
    let down_right = is_needle("MAS", table, center + Vec2(-1, -1), Vec2(1, 1))
        || is_needle("SAM", table, center + Vec2(-1, -1), Vec2(1, 1));
    let up_right = is_needle("MAS", table, center + Vec2(1, -1), Vec2(-1, 1))
        || is_needle("SAM", table, center + Vec2(1, -1), Vec2(-1, 1));
    down_right && up_right
}

const INPUT: &str = "data/y2024/d04/input";

/// ```
/// use advent_of_code::aoc::y2024::d04::part1;
/// assert_eq!(part1(), 2543);
/// ```
pub fn part1() -> usize {
    let hay = std::fs::read_to_string(INPUT).unwrap();
    let needle: &str = "XMAS";
    count_word(needle, &hay)
}

/// ```
/// use advent_of_code::aoc::y2024::d04::part2;
/// assert_eq!(part2(), 1930);
/// ```
pub fn part2() -> usize {
    let hay = std::fs::read_to_string(INPUT).unwrap();
    let hay = table::read_rows_chars(&hay);
    get_origins(&hay)
        .into_iter()
        .filter(|origin| is_x_mas(&hay, *origin))
        .count()
}
