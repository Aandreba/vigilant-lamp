use std::ops::{Add, Sub, Mul, Div, Deref};
use derive_more::{AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use num::{Float, Num};

use crate::shaders::UniformValue;

#[derive(Neg, AddAssign, SubAssign, MulAssign, DivAssign, Debug, PartialEq, Eq, Clone)]
pub struct EucVec2<T: Num + Copy> {
    pub x: T,
    pub y: T
}

pub type EucVecu2 = EucVec2<u64>;
pub type EucVeci2 = EucVec2<i64>;
pub type EucVecf2 = EucVec2<f32>;
pub type EucVecd2 = EucVec2<f64>;

impl<T: Num + Copy> EucVec2<T> {
    pub fn new (x: T, y: T) -> EucVec2<T> {
        EucVec2{x, y}
    }

    pub fn norm2 (&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn norm (&self) -> T where T: Float {
        self.x.hypot(self.y)
    }

    pub fn unit (&self) -> EucVec2<T> where T: Float {
        let norm = self.norm();
        EucVec2::new(self.x / norm, self.y / norm)
    }

    pub fn dot (self, rhs: EucVec2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

// VECTOR - VECTOR
impl<T: Num + Copy> Add<EucVec2<T>> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn add(self, rhs: EucVec2<T>) -> Self::Output {
       EucVec2::new(
           self.x + rhs.x,
           self.y + rhs.y
        )
    }
}

impl<T: Num + Copy> Sub<EucVec2<T>> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn sub(self, rhs: EucVec2<T>) -> Self::Output {
       EucVec2::new(
           self.x - rhs.x,
           self.y - rhs.y
        )
    }
}

impl<T: Num + Copy> Mul<EucVec2<T>> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn mul(self, rhs: EucVec2<T>) -> Self::Output {
       EucVec2::new(
           self.x * rhs.x,
           self.y * rhs.y
        )
    }
}

impl<T: Num + Copy> Div<EucVec2<T>> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn div(self, rhs: EucVec2<T>) -> Self::Output {
       EucVec2::new(
           self.x / rhs.x,
           self.y / rhs.y
        )
    }
}

// VECTOR - SCALAR
impl<T: Num + Copy> Add<T> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn add(self, rhs: T) -> Self::Output {
       EucVec2::new(
           self.x + rhs,
           self.y + rhs
        )
    }
}

impl<T: Num + Copy> Sub<T> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn sub(self, rhs: T) -> Self::Output {
       EucVec2::new(
           self.x - rhs,
           self.y - rhs
        )
    }
}

impl<T: Num + Copy> Mul<T> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
       EucVec2::new(
           self.x * rhs,
           self.y * rhs
        )
    }
}

impl<T: Num + Copy> Div<T> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn div(self, rhs: T) -> Self::Output {
       EucVec2::new(
           self.x / rhs,
           self.y / rhs
        )
    }
}

// OTHER TRAITS
impl UniformValue for EucVecf2 {
    fn set_to_program<P: crate::shaders::Program> (&self, program: &P, key: &P::Uniform) -> bool {
        program.set_float_vec2(key, self);
        true
    }
}

impl UniformValue for EucVecd2 {
    fn set_to_program<P: crate::shaders::Program> (&self, program: &P, key: &P::Uniform) -> bool {
        program.set_double_vec2(key, self);
        true
    }
}

impl<T: Num + Copy + Default> Default for EucVec2<T> {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}