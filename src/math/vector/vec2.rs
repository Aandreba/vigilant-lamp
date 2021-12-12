use std::ops::{Add, Sub, Mul, Div};
use derive_more::{AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use num::{Float, Num};

#[derive(Neg, AddAssign, SubAssign, MulAssign, DivAssign, Debug)]
pub struct EucVec2<T: Num> {
    pub x: T,
    pub y: T
}

impl<T: Num> EucVec2<T> {
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
impl<T: Num> Add<EucVec2<T>> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn add(self, rhs: EucVec2<T>) -> Self::Output {
       EucVec2::new(
           self.x + rhs.x,
           self.y + rhs.y
        )
    }
}

impl<T: Num> Sub<EucVec2<T>> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn sub(self, rhs: EucVec2<T>) -> Self::Output {
       EucVec2::new(
           self.x - rhs.x,
           self.y - rhs.y
        )
    }
}

impl<T: Num> Mul<EucVec2<T>> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn mul(self, rhs: EucVec2<T>) -> Self::Output {
       EucVec2::new(
           self.x * rhs.x,
           self.y * rhs.y
        )
    }
}

impl<T: Num> Div<EucVec2<T>> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn div(self, rhs: EucVec2<T>) -> Self::Output {
       EucVec2::new(
           self.x / rhs.x,
           self.y / rhs.y
        )
    }
}

// VECTOR - SCALAR
impl<T: Num> Add<T> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn add(self, rhs: T) -> Self::Output {
       EucVec2::new(
           self.x + rhs,
           self.y + rhs
        )
    }
}

impl<T: Num> Sub<T> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn sub(self, rhs: T) -> Self::Output {
       EucVec2::new(
           self.x - rhs,
           self.y - rhs
        )
    }
}

impl<T: Num> Mul<T> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
       EucVec2::new(
           self.x * rhs,
           self.y * rhs
        )
    }
}

impl<T: Num> Div<T> for EucVec2<T> {
    type Output = EucVec2<T>;

    fn div(self, rhs: T) -> Self::Output {
       EucVec2::new(
           self.x / rhs,
           self.y / rhs
        )
    }
}