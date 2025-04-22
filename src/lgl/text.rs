use std::fmt::Display;

/// Convert a slice into a one-line String.
pub fn slice_to_line(v: &[impl Display], sep: &str) -> String {
    let strings: Vec<String> = v.iter().map(|x| format!("{x}")).collect();
    strings.join(sep)
}
