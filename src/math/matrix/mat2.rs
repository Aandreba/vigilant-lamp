use std::ops::Mul;

use derive_more::{Neg, Add, Sub, AddAssign, SubAssign};
use num::Num;

use crate::vector::EucVec2;

// MAT2
#[derive(Neg, Add, Sub, AddAssign, SubAssign, Debug, PartialEq, Eq, Clone)]
pub struct Mat2<T: Num + Copy> {
    pub x: EucVec2<T>,
    pub y: EucVec2<T>
}

impl<T: Num + Copy> Mat2<T> {
    pub fn new (x: EucVec2<T>, y: EucVec2<T>) -> Mat2<T> {
        Mat2{x, y}
    }

    pub fn of (xx: T, xy: T, yx: T, yy: T) -> Mat2<T> {
        Mat2::new(EucVec2::new(xx, xy), EucVec2::new(yx, yy))
    }

    pub fn transp (&self) -> Mat2<T> {
        Mat2::of(
            self.x.x, self.y.x,
            self.x.y, self.y.y
        )
    }

    pub fn tr (&self) -> T {
        self.x.x + self.y.y
    }

    pub fn det (&self) -> T {
        self.x.x * self.y.y - self.x.y * self.y.x
    }
}


// MULTIPLICATION
impl<T: Num + Copy> Mul<Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn mul (self, rhs: Mat2<T>) -> Self::Output {
        Mat2::of(
            self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            self.x.x * rhs.x.y + self.x.y * rhs.y.y,

            self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            self.y.x * rhs.x.y + self.y.y * rhs.y.y
        )
    }
}

impl<T: Num + Copy> Mul<EucVec2<T>> for Mat2<T> {
    type Output = EucVec2<T>;

    fn mul (self, rhs: EucVec2<T>) -> Self::Output {
        EucVec2::new(
            self.x.x * rhs.x + self.x.y * rhs.y,
            self.y.x * rhs.x + self.y.y * rhs.y
        )
    }
}

impl<T: Num + Copy> Mul<T> for Mat2<T> {
    type Output = Mat2<T>;

    fn mul (self, rhs: T) -> Self::Output {
        Mat2::new(
            self.x * rhs,
            self.y * rhs
        )
    }
}

#[cfg(test)]
#[test]
fn mul () {
    let alpha = Mat2::of(1., 2., 3., 4.);
    let beta = Mat2::of(5., 6., 7., 8.);
    assert_eq!(alpha * beta, Mat2::of(19., 22., 43., 50.))
}