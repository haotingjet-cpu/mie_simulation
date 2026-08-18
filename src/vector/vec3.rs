use super::{Cross, Dot, Norm};
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[repr(align(32))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec3<T> {
    vec: [T; 3],
}

impl<T> Add<Vec3<T>> for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            vec: [
                self.vec[0] + rhs.vec[0],
                self.vec[1] + rhs.vec[1],
                self.vec[2] + rhs.vec[2],
            ],
        }
    }
}

impl<'a, 'b, T> Add<&'b Vec3<T>> for &'a Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: &'b Vec3<T>) -> Self::Output {
        Vec3 {
            vec: [
                self.vec[0] + rhs.vec[0],
                self.vec[1] + rhs.vec[1],
                self.vec[2] + rhs.vec[2],
            ],
        }
    }
}

impl<'a, T> Add<Vec3<T>> for &'a mut Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = &'a mut Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        self.vec[0] = self.vec[0] + rhs.vec[0];
        self.vec[1] = self.vec[1] + rhs.vec[1];
        self.vec[2] = self.vec[2] + rhs.vec[2];

        self
    }
}

impl<'a, T> Mul<T> for &'a mut Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = &'a mut Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        self.vec[0] = self.vec[0] * rhs;
        self.vec[1] = self.vec[1] * rhs;
        self.vec[2] = self.vec[2] * rhs;

        self
    }
}

impl<T> Sub<Vec3<T>> for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            vec: [
                self.vec[0] - rhs.vec[0],
                self.vec[1] - rhs.vec[1],
                self.vec[2] - rhs.vec[2],
            ],
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            vec: [self.vec[0] * rhs, self.vec[1] * rhs, self.vec[2] * rhs],
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            vec: [self.vec[0] / rhs, self.vec[1] / rhs, self.vec[2] / rhs],
        }
    }
}

impl<T> Dot<T> for Vec3<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    fn dot(&self, other: &Self) -> T {
        self.vec[0] * other.vec[0] + self.vec[1] * other.vec[1] + self.vec[2] * other.vec[2]
    }
}

impl<T> Vec3<T> {
    pub(crate) fn new(i0: T, i1: T, i2: T) -> Self {
        Self { vec: [i0, i1, i2] }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

impl<T> Norm<T> for Vec3<T>
where
    T: num_traits::Float + Copy,
{
    fn norm(&self) -> T {
        (self.vec[0] * self.vec[0] + self.vec[1] * self.vec[1] + self.vec[2] * self.vec[2]).sqrt()
    }

    fn normalize(self) -> Self {
        self / self.norm()
    }
}

impl<T> Cross<T> for Vec3<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Copy,
{
    fn cross(&self, rhs: &Self) -> Self {
        Vec3::new(
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        )
    }
}
