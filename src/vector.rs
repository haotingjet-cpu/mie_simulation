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

pub(crate) trait Dot<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    fn dot(&self, rhs: &Self) -> T;
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

// ======================================================================================================

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

impl<T> Vec3<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    fn abs(&self) -> T {
        self.vec[0] * self.vec[0] + self.vec[1] * self.vec[1] + self.vec[2] * self.vec[2]
    }
}

// ===========================================================================================

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

impl Matrix4<f32> {
    pub(crate) fn identity_mat() -> Self {
        Self {
            rows: [
                Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 1.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
}

impl Matrix4<f64> {
    pub(crate) fn identity_mat() -> Self {
        Self {
            rows: [
                Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 1.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
}

impl Matrix4<i32> {
    pub(crate) fn identity_mat() -> Self {
        Self {
            rows: [
                Vec4::new(1, 0, 0, 0),
                Vec4::new(0, 1, 0, 0),
                Vec4::new(0, 0, 1, 0),
                Vec4::new(0, 0, 0, 1),
            ],
        }
    }
}

impl Matrix4<i64> {
    pub(crate) fn identity_mat() -> Self {
        Self {
            rows: [
                Vec4::new(1, 0, 0, 0),
                Vec4::new(0, 1, 0, 0),
                Vec4::new(0, 0, 1, 0),
                Vec4::new(0, 0, 0, 1),
            ],
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
