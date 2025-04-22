use std::ops::{Add, Mul, Sub};

/// Signed 2D index
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Ix2s(pub isize, pub isize);

impl Add for Ix2s {
    type Output = Ix2s;
    fn add(self, rhs: Self) -> Self::Output {
        Ix2s(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Ix2s {
    type Output = Ix2s;
    fn sub(self, rhs: Self) -> Self::Output {
        Ix2s(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<isize> for Ix2s {
    type Output = Ix2s;
    fn mul(self, rhs: isize) -> Self::Output {
        Ix2s(self.0 * rhs, self.1 * rhs)
    }
}

impl TryFrom<(usize, usize)> for Ix2s {
    type Error = <isize as TryFrom<usize>>::Error;
    fn try_from((u0, u1): (usize, usize)) -> Result<Self, Self::Error> {
        let i0: isize = u0.try_into()?;
        let i1: isize = u1.try_into()?;
        Ok(Ix2s(i0, i1))
    }
}

impl TryFrom<ndarray::Ix2> for Ix2s {
    type Error = <isize as TryFrom<usize>>::Error;
    fn try_from(index: ndarray::Ix2) -> Result<Self, Self::Error> {
        let i0: isize = index[0].try_into()?;
        let i1: isize = index[1].try_into()?;
        Ok(Ix2s(i0, i1))
    }
}

impl TryInto<(usize, usize)> for Ix2s {
    type Error = <isize as TryInto<usize>>::Error;
    fn try_into(self) -> Result<(usize, usize), Self::Error> {
        let u0: usize = self.0.try_into()?;
        let u1: usize = self.1.try_into()?;
        Ok((u0, u1))
    }
}

impl TryInto<ndarray::Ix2> for Ix2s {
    type Error = <isize as TryInto<usize>>::Error;
    fn try_into(self) -> Result<ndarray::Ix2, Self::Error> {
        let u0: usize = self.0.try_into()?;
        let u1: usize = self.1.try_into()?;
        Ok(ndarray::Ix2(u0, u1))
    }
}
