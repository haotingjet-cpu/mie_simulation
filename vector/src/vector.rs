use std::ops::{Add, Div, Mul, Sub};

// ======================================================================================================
pub trait Dot<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    fn dot(&self, rhs: &Self) -> T;
}

// =======================================================================
pub trait Cross<T>
where
    T: Sub<Output = T> + Mul<Output = T> + Copy,
{
    fn cross(&self, rhs: &Self) -> Self;
}

// ===========================================================================================

pub trait Norm<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + Div<Output = T>,
{
    fn norm(&self) -> T;
    fn normalize(self) -> Self;
}
