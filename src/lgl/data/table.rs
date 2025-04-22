use std::{fmt::Debug, fs, path::Path, str::FromStr};

/// Read whitespace-separated cells into a table.
pub fn to_table_whitespace(s: &str) -> Vec<Vec<&str>> {
    s.lines().map(|l| l.split_whitespace().collect()).collect()
}

pub fn to_char_table(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

pub fn to_table(s: &str, sep: char) -> Vec<Vec<&str>> {
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
    to_table_whitespace(input)
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

pub fn read_rows<T>(input_file: impl AsRef<Path>) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    parse_rows(&fs::read_to_string(input_file).unwrap())
}

pub fn read_cols<T>(input_file: impl AsRef<Path>) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    transpose(parse_rows(&fs::read_to_string(input_file).unwrap()))
}
