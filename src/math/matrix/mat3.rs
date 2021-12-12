use std::ops::{Add, Index, IndexMut};
use num::Num;

// MAT2
pub struct Mat3<T: Num> (
    T, T, T, 
    T, T, T, 
    T, T, T
);

impl<T: Num> Mat3<T> {
    pub fn new (xx: T, xy: T, xz: T, yx: T, yy: T, yz: T, zx: T, zy: T, zz: T) -> Mat3<T> {
        Mat3(
            xx, xy, xz, 
            yx, yy, yz,
            zx, zy, zz
        )
    }
}

// MATRIX - MATRIX
impl<T: Num> Add<Mat3<T>> for Mat3<T>{
    type Output = Mat3<T>;

    fn add (self, rhs: Mat3<T>) -> Self::Output {
        Mat3(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
            self.4 + rhs.4,
            self.5 + rhs.5,
            self.6 + rhs.6,
            self.7 + rhs.7,
            self.8 + rhs.8,
        )
    }
}

// MATRIX - SCALAR
impl<T: Num> Add<T> for Mat3<T>{
    type Output = Mat3<T>;

    fn add (self, rhs: T) -> Self::Output {
        Mat3(
            self.0 + rhs,
            self.1 + rhs,
            self.2 + rhs,
            self.3 + rhs,
            self.4 + rhs,
            self.5 + rhs,
            self.6 + rhs,
            self.7 + rhs,
            self.8 + rhs,
        )
    }
}