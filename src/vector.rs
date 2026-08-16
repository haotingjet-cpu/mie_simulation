use std::ops::{Add, Mul};
pub(crate) mod vec4;
pub(crate) use self::vec4::Vec4;
pub(crate) mod matrix4;
pub(crate) use self::matrix4::Matrix4;
pub(crate) mod vec3;
pub(crate) use self::vec3::Vec3;

// ======================================================================================================
pub(crate) trait Dot<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    fn dot(&self, rhs: &Self) -> T;
}

// ===========================================================================================

pub(crate) trait Norm<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    fn norm(&self) -> T;
}
