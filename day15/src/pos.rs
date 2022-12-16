use derive_more::{Add, AddAssign, From, Into, Sub, SubAssign};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Sub, SubAssign, From, Into)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    fn op<F: Fn(i64, i64) -> i64>(&self, other: &Self, op: &F) -> Self {
        (op(self.x, other.x), op(self.y, other.y)).into()
    }

    pub fn min(&self, other: &Self) -> Self {
        self.op(other, &i64::min)
    }

    pub fn max(&self, other: &Self) -> Self {
        self.op(other, &i64::max)
    }

    pub fn distance(&self, other: &Self) -> i64 {
        (self - other).norm()
    }

    pub fn norm(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

// == Boring trivial operator implementation below ==

impl Add<&Pos> for &Pos {
    type Output = Pos;

    fn add(self, rhs: &Pos) -> Self::Output {
        *self + *rhs
    }
}

impl Sub<&Pos> for &Pos {
    type Output = Pos;

    fn sub(self, rhs: &Pos) -> Self::Output {
        *self - *rhs
    }
}

impl Add<i64> for Pos {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        self.op(&(rhs, rhs).into(), &i64::add)
    }
}

impl Sub<i64> for Pos {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        self.op(&(rhs, rhs).into(), &i64::sub)
    }
}

impl Mul<Pos> for i64 {
    type Output = Pos;

    fn mul(self, rhs: Pos) -> Self::Output {
        rhs.op(&(self, self).into(), &i64::mul)
    }
}
