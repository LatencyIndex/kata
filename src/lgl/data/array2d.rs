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

pub fn get<T>(arr: &Array2<T>, index: Ix2s) -> Option<&T> {
    let i: usize = index.0.try_into().ok()?;
    let j: usize = index.1.try_into().ok()?;
    arr.get((i, j))
}

pub fn get_mut<T>(arr: &mut Array2<T>, index: Ix2s) -> Option<&mut T> {
    let i: usize = index.0.try_into().ok()?;
    let j: usize = index.1.try_into().ok()?;
    arr.get_mut((i, j))
}
