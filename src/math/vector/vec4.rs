use std::ops::{Add, Sub, Mul, Div};
use derive_more::{AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use num::{Float, Num};

use crate::shaders::{UniformValue, Program};

use super::{EucVec3, EucVec2};

#[derive(Neg, AddAssign, SubAssign, MulAssign, DivAssign, Debug, PartialEq, Eq, Clone)]
pub struct EucVec4<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

pub type EucVecu4 = EucVec4<u64>;
pub type EucVeci4 = EucVec4<i64>;
pub type EucVecf4 = EucVec4<f32>;
pub type EucVecd4 = EucVec4<f64>;

impl<T: Num + Copy> EucVec4<T> {
    pub fn new (x: T, y: T, z: T, w: T) -> EucVec4<T> {
        EucVec4{x, y, z, w}
    }

    pub fn norm2 (&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn norm (&self) -> T where T: Float {
        self.norm2().sqrt()
    }

    pub fn unit (&self) -> EucVec4<T> where T: Float {
        let norm = self.norm();
        EucVec4::new(self.x / norm, self.y / norm, self.z / norm, self.w / norm)
    }

    pub fn dot (self, rhs: EucVec4<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn xy (&self) -> EucVec2<T> {
        EucVec2::new(self.x, self.y)
    }

    pub fn xyz (&self) -> EucVec3<T> {
        EucVec3::new(self.x, self.y, self.z)
    }
}

// VECTOR - VECTOR
impl<T: Num + Copy> Add<EucVec4<T>> for EucVec4<T> {
    type Output = EucVec4<T>;

    fn add(self, rhs: EucVec4<T>) -> Self::Output {
       EucVec4::new(
           self.x + rhs.x,
           self.y + rhs.y,
           self.z + rhs.z,
           self.w + rhs.w
        )
    }
}

impl<T: Num + Copy> Sub<EucVec4<T>> for EucVec4<T> {
    type Output = EucVec4<T>;

    fn sub(self, rhs: EucVec4<T>) -> Self::Output {
       EucVec4::new(
           self.x - rhs.x,
           self.y - rhs.y,
           self.z - rhs.z,
           self.w - rhs.w
        )
    }
}

impl<T: Num + Copy> Mul<EucVec4<T>> for EucVec4<T> {
    type Output = EucVec4<T>;

    fn mul(self, rhs: EucVec4<T>) -> Self::Output {
       EucVec4::new(
           self.x * rhs.x,
           self.y * rhs.y,
           self.z * rhs.z,
           self.w * rhs.w
        )
    }
}

impl<T: Num + Copy> Div<EucVec4<T>> for EucVec4<T> {
    type Output = EucVec4<T>;

    fn div(self, rhs: EucVec4<T>) -> Self::Output {
       EucVec4::new(
           self.x / rhs.x,
           self.y / rhs.y,
           self.z / rhs.z,
           self.w / rhs.w
        )
    }
}

// VECTOR - SCALAR
impl<T: Num + Copy> Add<T> for EucVec4<T> {
    type Output = EucVec4<T>;

    fn add(self, rhs: T) -> Self::Output {
       EucVec4::new(
           self.x + rhs,
           self.y + rhs,
           self.z + rhs,
           self.w + rhs
        )
    }
}

impl<T: Num + Copy> Sub<T> for EucVec4<T> {
    type Output = EucVec4<T>;

    fn sub(self, rhs: T) -> Self::Output {
       EucVec4::new(
           self.x - rhs,
           self.y - rhs,
           self.z - rhs,
           self.w - rhs
        )
    }
}

impl<T: Num + Copy> Mul<T> for EucVec4<T> {
    type Output = EucVec4<T>;

    fn mul(self, rhs: T) -> Self::Output {
       EucVec4::new(
           self.x * rhs,
           self.y * rhs,
           self.z * rhs,
           self.w * rhs
        )
    }
}

impl<T: Num + Copy> Div<T> for EucVec4<T> {
    type Output = EucVec4<T>;

    fn div(self, rhs: T) -> Self::Output {
       EucVec4::new(
           self.x / rhs,
           self.y / rhs,
           self.z / rhs,
           self.w / rhs
        )
    }
}

// OTHER TRAITS
impl UniformValue for EucVecf4 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_float_vec4(&x.clone(), self)
        }

        true
    }
}

impl UniformValue for EucVecd4 {
    fn set_to_program<P: Program> (&self, program: &mut P, key: &str) -> bool {
        match program.get_uniform(key) {
            None => return false,
            Some(x) => program.set_double_vec4(&x.clone(), self)
        }

        true
    }
}

impl<T: Num + Copy + Default> Default for EucVec4<T> {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), z: Default::default(), w: Default::default() }
    }
}