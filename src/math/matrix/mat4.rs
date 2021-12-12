use std::ops::{Add};
use num::Num;

// MAT2
pub struct Mat4<T: Num> (
    T, T, T, T,
    T, T, T, T,
    T, T, T, T,
    T, T, T, T
);

impl<T: Num> Mat4<T> {
    pub fn new (xx: T, xy: T, xz: T, xw: T, yx: T, yy: T, yz: T, yw: T, zx: T, zy: T, zz: T, zw: T, wx: T, wy: T, wz: T, ww: T) -> Mat4<T> {
        Mat4(
            xx, xy, xz, xw,
            yx, yy, yz, yw,
            zx, zy, zz, zw,
            wx, wy, wz, ww
        )
    }
}

// MATRIX - MATRIX
impl<T: Num> Add<Mat4<T>> for Mat4<T>{
    type Output = Mat4<T>;

    fn add (self, rhs: Mat4<T>) -> Self::Output {
        Mat4(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
            self.4 + rhs.4,
            self.5 + rhs.5,
            self.6 + rhs.6,
            self.7 + rhs.7,
            self.8 + rhs.8,
            self.9 + rhs.9,
            self.10 + rhs.10,
            self.11 + rhs.11,
            self.12 + rhs.12,
            self.13 + rhs.13,
            self.14 + rhs.14,
            self.15 + rhs.15
        )
    }
}

// MATRIX - SCALAR
impl<T: Num> Add<T> for Mat4<T>{
    type Output = Mat4<T>;

    fn add (self, rhs: T) -> Self::Output {
        Mat4(
            self.0 + rhs,
            self.1 + rhs,
            self.2 + rhs,
            self.3 + rhs,
            self.4 + rhs,
            self.5 + rhs,
            self.6 + rhs,
            self.7 + rhs,
            self.8 + rhs,
            self.9 + rhs,
            self.10 + rhs,
            self.11 + rhs,
            self.12 + rhs,
            self.13 + rhs,
            self.14 + rhs,
            self.15 + rhs
        )
    }
}