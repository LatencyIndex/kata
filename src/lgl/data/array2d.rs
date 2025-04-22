use std::fmt::Debug;

use crate::lgl::data::index::Ix2s;
pub use ndarray::{Array2, Ix2};

/// Construct a 2D array from a rectangular table
pub fn to_ndarray<T: Clone>(table: &[Vec<T>]) -> Array2<T> {
    let n = table.len();
    let m = table.first().map(|v| v.len()).unwrap_or(0);
    // Table must be rectangular
    assert!(table.iter().all(|v| v.len() == m));
    let flat_table: Vec<T> = table.iter().flat_map(Clone::clone).collect();
    Array2::from_shape_vec((n, m), flat_table).unwrap()
}

pub fn contains<T>(arr: &Array2<T>, pos: Ix2s) -> bool {
    let pos: Option<Ix2> = pos.try_into().ok();
    if let Some(pos) = pos {
        let shape = arr.raw_dim();
        pos[0] < shape[0] && pos[1] < shape[1]
    } else {
        false
    }
}

pub fn get<T>(arr: &Array2<T>, index: Ix2s) -> Option<&T> {
    let index: Ix2 = index.try_into().ok()?;
    arr.get(index)
}

pub fn get_mut<T>(arr: &mut Array2<T>, index: Ix2s) -> Option<&mut T> {
    let index: Ix2 = index.try_into().ok()?;
    arr.get_mut(index)
}

pub fn compact_display<T: Debug>(arr: &Array2<T>, sep: &str) -> String {
    let lines: Vec<String> = arr
        .rows()
        .into_iter()
        .map(|row| {
            let strings: Vec<String> = row.iter().map(|x| format!("{x:?}")).collect();
            strings.join(sep)
        })
        .collect();
    lines.join("\n")
}
