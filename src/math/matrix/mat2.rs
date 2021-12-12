use std::ops::{Add};
use num::Num;

// MAT2
pub struct Mat2<T: Num> (T, T, T, T);

impl<T: Num> Mat2<T> {
    pub fn new (xx: T, xy: T, yx: T, yy: T) -> Mat2<T> {
        Mat2(xx, xy, yx, yy)
    }
}

// MATRIX - MATRIX
impl<T: Num> Add<Mat2<T>> for Mat2<T>{
    type Output = Mat2<T>;

    fn add (self, rhs: Mat2<T>) -> Self::Output {
        Mat2(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3
        )
    }
}

// MATRIX - SCALAR
impl<T: Num> Add<T> for Mat2<T>{
    type Output = Mat2<T>;

    fn add (self, rhs: T) -> Self::Output {
        Mat2(
            self.0 + rhs,
            self.1 + rhs,
            self.2 + rhs,
            self.3 + rhs
        )
    }
}