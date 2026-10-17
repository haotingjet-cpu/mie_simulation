use super::{Dot, Vec4};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[repr(align(32))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Matrix4<T> {
    rows: [Vec4<T>; 4],
}

impl<T> Matrix4<T> {
    pub(crate) fn new(i0: Vec4<T>, i1: Vec4<T>, i2: Vec4<T>, i3: Vec4<T>) -> Matrix4<T> {
        Matrix4 {
            rows: [i0, i1, i2, i3],
        }
    }
}

impl<T> Add<Matrix4<T>> for Matrix4<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Matrix4<T>;
    fn add(self, rhs: Matrix4<T>) -> Self::Output {
        let mux = std::array::from_fn(|r| self.rows[r] + rhs.rows[r]);

        Matrix4 { rows: mux }
    }
}

impl<T> Sub<Matrix4<T>> for Matrix4<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Matrix4<T>;
    fn sub(self, rhs: Matrix4<T>) -> Self::Output {
        let mux = std::array::from_fn(|r| self.rows[r] - rhs.rows[r]);

        Matrix4 { rows: mux }
    }
}

impl<T> Mul<T> for Matrix4<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Matrix4<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let mux = std::array::from_fn(|r| self.rows[r] * rhs);

        Matrix4 { rows: mux }
    }
}

impl<T> Matrix4<T>
where
    T: Copy,
{
    pub(crate) fn transpose(self) -> Self {
        let vec0 = self.rows[0].as_array();
        let vec1 = self.rows[1].as_array();
        let vec2 = self.rows[2].as_array();
        let vec3 = self.rows[3].as_array();

        Self {
            rows: [
                Vec4::new(vec0[0], vec1[0], vec2[0], vec3[0]),
                Vec4::new(vec0[1], vec1[1], vec2[1], vec3[1]),
                Vec4::new(vec0[2], vec1[2], vec2[2], vec3[2]),
                Vec4::new(vec0[3], vec1[3], vec2[3], vec3[3]),
            ],
        }
    }
}

impl<T> Mul<Vec4<T>> for Matrix4<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Vec4<T>;
    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        let mat = self.rows;
        Vec4::new(
            mat[0].dot(&rhs),
            mat[1].dot(&rhs),
            mat[2].dot(&rhs),
            mat[3].dot(&rhs),
        )
    }
}

impl<T> Mul<Matrix4<T>> for Matrix4<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Matrix4<T>;
    fn mul(self, rhs: Matrix4<T>) -> Self::Output {
        let rhs = rhs.transpose().rows;
        let mat = self.rows;
        Self::new(
            Vec4::new(
                mat[0].dot(&rhs[0]),
                mat[0].dot(&rhs[1]),
                mat[0].dot(&rhs[2]),
                mat[0].dot(&rhs[3]),
            ),
            Vec4::new(
                mat[1].dot(&rhs[0]),
                mat[1].dot(&rhs[1]),
                mat[1].dot(&rhs[2]),
                mat[1].dot(&rhs[3]),
            ),
            Vec4::new(
                mat[2].dot(&rhs[0]),
                mat[2].dot(&rhs[1]),
                mat[2].dot(&rhs[2]),
                mat[2].dot(&rhs[3]),
            ),
            Vec4::new(
                mat[3].dot(&rhs[0]),
                mat[3].dot(&rhs[1]),
                mat[3].dot(&rhs[2]),
                mat[3].dot(&rhs[3]),
            ),
        )
    }
}

impl<T> Index<usize> for Matrix4<T> {
    type Output = Vec4<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T> IndexMut<usize> for Matrix4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
