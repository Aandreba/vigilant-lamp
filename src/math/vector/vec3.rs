use std::ops::{Add, Index, IndexMut, Sub, Mul, Div};
use derive_more::{AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use num::{Float, Num};

use super::EucVec2;

#[derive(Neg, AddAssign, SubAssign, MulAssign, DivAssign, Debug, PartialEq, Eq, Clone)]
pub struct EucVec3<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub z: T
}

pub type EucVecu3 = EucVec3<u64>;
pub type EucVeci3 = EucVec3<i64>;
pub type EucVecf3 = EucVec3<f32>;
pub type EucVecd3 = EucVec3<f64>;

impl<T: Num + Copy> EucVec3<T> {
    pub fn new (x: T, y: T, z: T) -> EucVec3<T> {
        EucVec3{x, y, z}
    }

    pub fn norm2 (&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm (&self) -> T where T: Float {
        self.norm2().sqrt()
    }

    pub fn unit (&self) -> EucVec3<T> where T: Float {
        let norm = self.norm();
        EucVec3::new(self.x / norm, self.y / norm, self.z / norm)
    }

    pub fn dot (self, rhs: EucVec3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross (self, rhs: EucVec3<T>) -> EucVec3<T> {
        EucVec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }

    pub fn xy (&self) -> EucVec2<T> {
        EucVec2::new(self.x, self.y)
    }
}

// VECTOR - VECTOR
impl<T: Num + Copy> Add<EucVec3<T>> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn add(self, rhs: EucVec3<T>) -> Self::Output {
       EucVec3::new(
           self.x + rhs.x,
           self.y + rhs.y,
           self.z + rhs.z
        )
    }
}

impl<T: Num + Copy> Sub<EucVec3<T>> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn sub(self, rhs: EucVec3<T>) -> Self::Output {
       EucVec3::new(
           self.x - rhs.x,
           self.y - rhs.y,
           self.z - rhs.z
        )
    }
}

impl<T: Num + Copy> Mul<EucVec3<T>> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn mul(self, rhs: EucVec3<T>) -> Self::Output {
       EucVec3::new(
           self.x * rhs.x,
           self.y * rhs.y,
           self.z * rhs.z
        )
    }
}

impl<T: Num + Copy> Div<EucVec3<T>> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn div(self, rhs: EucVec3<T>) -> Self::Output {
       EucVec3::new(
           self.x / rhs.x,
           self.y / rhs.y,
           self.z / rhs.z
        )
    }
}

// VECTOR - SCALAR
impl<T: Num + Copy> Add<T> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn add(self, rhs: T) -> Self::Output {
       EucVec3::new(
           self.x + rhs,
           self.y + rhs,
           self.z + rhs
        )
    }
}

impl<T: Num + Copy> Sub<T> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
       EucVec3::new(
           self.x - rhs,
           self.y - rhs,
           self.z - rhs
        )
    }
}

impl<T: Num + Copy> Mul<T> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
       EucVec3::new(
           self.x * rhs,
           self.y * rhs,
           self.z * rhs
        )
    }
}

impl<T: Num + Copy> Div<T> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn div(self, rhs: T) -> Self::Output {
       EucVec3::new(
           self.x / rhs,
           self.y / rhs,
           self.z / rhs
        )
    }
}

// OTHER TRAITS
impl<T: Num + Copy + Default> Default for EucVec3<T> {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), z: Default::default() }
    }
}