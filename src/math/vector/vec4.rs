use std::ops::{Add, Sub, Mul, Div};
use derive_more::{AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use num::{Float, Num};

#[derive(Neg, AddAssign, SubAssign, MulAssign, DivAssign, Debug)]
pub struct EucVec4<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T: Num> EucVec4<T> {
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
}

// VECTOR - VECTOR
impl<T: Num> Add<EucVec4<T>> for EucVec4<T> {
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

impl<T: Num> Sub<EucVec4<T>> for EucVec4<T> {
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

impl<T: Num> Mul<EucVec4<T>> for EucVec4<T> {
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

impl<T: Num> Div<EucVec4<T>> for EucVec4<T> {
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
impl<T: Num> Add<T> for EucVec4<T> {
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

impl<T: Num> Sub<T> for EucVec4<T> {
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

impl<T: Num> Mul<T> for EucVec4<T> {
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

impl<T: Num> Div<T> for EucVec4<T> {
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