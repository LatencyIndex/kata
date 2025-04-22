use std::ops::{Add, Mul};

/// Signed 2D index
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Ix2s(pub isize, pub isize);

impl Ix2s {
    pub fn from_upair((i, j): (usize, usize)) -> Ix2s {
        Ix2s(i.try_into().unwrap(), j.try_into().unwrap())
    }
}

impl Add for Ix2s {
    type Output = Ix2s;
    fn add(self, rhs: Self) -> Self::Output {
        Ix2s(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<isize> for Ix2s {
    type Output = Ix2s;
    fn mul(self, rhs: isize) -> Self::Output {
        Ix2s(self.0 * rhs, self.1 * rhs)
    }
}
