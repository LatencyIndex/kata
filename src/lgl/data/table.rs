use std::{fmt::Debug, ops::Add, str::FromStr};

/// Convert text to a row-major table, with whitespace separated cells.
pub fn read_rows_whitespace(s: &str) -> Vec<Vec<&str>> {
    s.lines().map(|l| l.split_whitespace().collect()).collect()
}

/// Convert text to a row-major table, where each char is its own cell.
pub fn read_rows_chars(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

/// Convert text to a row-major table, with cells separated by 'sep'.
pub fn read_rows<'a>(s: &'a str, sep: &str) -> Vec<Vec<&'a str>> {
    s.lines().map(|l| l.split(sep).collect()).collect()
}

pub fn transpose<T>(m: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut t: Vec<Vec<T>> = Vec::new();
    for row in m.into_iter() {
        for (c, x) in row.into_iter().enumerate() {
            loop {
                match t.get_mut(c) {
                    Some(v) => {
                        v.push(x);
                        break;
                    }
                    None => t.push(Vec::new()),
                }
            }
        }
    }
    t
}

pub fn parse<T>(table: Vec<Vec<&str>>) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    table
        .iter()
        .map(|col| col.iter().map(|x| x.parse().unwrap()).collect())
        .collect()
}

/// 2D index
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct I2(pub isize, pub isize);

impl Add for I2 {
    type Output = I2;
    fn add(self, rhs: Self) -> Self::Output {
        I2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub fn get<T>(table: &[Vec<T>], index: I2) -> Option<&T> {
    let i: usize = index.0.try_into().ok()?;
    let j: usize = index.1.try_into().ok()?;
    table.get(i).and_then(|v| v.get(j))
}

pub fn get_mut<T>(table: &mut [Vec<T>], index: I2) -> Option<&mut T> {
    let i: usize = index.0.try_into().ok()?;
    let j: usize = index.1.try_into().ok()?;
    table.get_mut(i).and_then(|v| v.get_mut(j))
}

/// All the valid indexes of the given table.
pub fn all_indexes<T>(table: &[Vec<T>]) -> Vec<I2> {
    table
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(move |(j, _)| I2(i as isize, j as isize))
        })
        .collect()
}
