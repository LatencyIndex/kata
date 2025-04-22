use crate::lgl::data::{index::Ix2s, table};

fn is_needle(needle: &str, table: &[Vec<char>], origin: Ix2s, dir: Ix2s) -> bool {
    let coords = (0..).map(|k| origin + dir * k);
    let word = coords
        .map_while(|p| table::get(table, p).copied())
        .take(needle.len());
    word.eq(needle.chars())
}

/// Counts how many times the word appears in the table.
/// Occurences can be vertical, horizontal, or diagonal, forwards and backwards.
fn count_word(needle: &str, hay: &str) -> usize {
    let dirs = [
        Ix2s(0, 1),
        Ix2s(0, -1),
        Ix2s(1, 0),
        Ix2s(-1, 0),
        Ix2s(1, 1),
        Ix2s(1, -1),
        Ix2s(-1, 1),
        Ix2s(-1, -1),
    ];
    let hay: Vec<Vec<char>> = table::read_rows_chars(hay);
    let mut hits = 0;
    for origin in table::all_indexes(&hay) {
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
fn is_x_mas(table: &[Vec<char>], center: Ix2s) -> bool {
    let down_right = is_needle("MAS", table, center + Ix2s(-1, -1), Ix2s(1, 1))
        || is_needle("SAM", table, center + Ix2s(-1, -1), Ix2s(1, 1));
    let up_right = is_needle("MAS", table, center + Ix2s(1, -1), Ix2s(-1, 1))
        || is_needle("SAM", table, center + Ix2s(1, -1), Ix2s(-1, 1));
    down_right && up_right
}

const INPUT: &str = "data/y2024/d04/input";

pub fn part1() -> usize {
    let hay = std::fs::read_to_string(INPUT).unwrap();
    let needle: &str = "XMAS";
    count_word(needle, &hay)
}

pub fn part2() -> usize {
    let hay = std::fs::read_to_string(INPUT).unwrap();
    let hay = table::read_rows_chars(&hay);
    table::all_indexes(&hay)
        .into_iter()
        .filter(|origin| is_x_mas(&hay, *origin))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2543);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1930);
    }
}
