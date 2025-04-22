use std::{fmt::Debug, str::FromStr};

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

pub fn parse_rows<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read_rows_whitespace(input)
        .iter()
        .map(|col| col.iter().map(|x| x.parse().unwrap()).collect())
        .collect()
}

pub fn parse_cols<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    transpose(parse_rows(input))
}

pub fn parse_table<T>(input: Vec<Vec<&str>>) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .iter()
        .map(|col| col.iter().map(|x| x.parse().unwrap()).collect())
        .collect()
}
