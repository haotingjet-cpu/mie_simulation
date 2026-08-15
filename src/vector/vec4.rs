use super::Dot;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[repr(align(32))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec4<T> {
    vec: [T; 4],
}

impl<T> Add<Vec4<T>> for Vec4<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec4 {
            vec: [
                self.vec[0] + rhs.vec[0],
                self.vec[1] + rhs.vec[1],
                self.vec[2] + rhs.vec[2],
                self.vec[3] + rhs.vec[3],
            ],
        }
    }
}

impl<T> Sub<Vec4<T>> for Vec4<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec4 {
            vec: [
                self.vec[0] - rhs.vec[0],
                self.vec[1] - rhs.vec[1],
                self.vec[2] - rhs.vec[2],
                self.vec[3] - rhs.vec[3],
            ],
        }
    }
}

impl<T> Mul<T> for Vec4<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec4 {
            vec: [
                self.vec[0] * rhs,
                self.vec[1] * rhs,
                self.vec[2] * rhs,
                self.vec[3] * rhs,
            ],
        }
    }
}

impl<T> Dot<T> for Vec4<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    fn dot(&self, other: &Self) -> T {
        self.vec[0] * other.vec[0]
            + self.vec[1] * other.vec[1]
            + self.vec[2] * other.vec[2]
            + self.vec[3] * other.vec[3]
    }
}
impl<T> Vec4<T>
where
    T: Copy,
{
    pub(crate) fn as_array(self) -> [T; 4] {
        self.vec
    }
}

impl<T> Vec4<T> {
    pub(crate) fn new(i0: T, i1: T, i2: T, i3: T) -> Self {
        Self {
            vec: [i0, i1, i2, i3],
        }
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T> IndexMut<usize> for Vec4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}
