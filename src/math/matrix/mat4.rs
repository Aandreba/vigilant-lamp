use std::ops::Mul;
use derive_more::{Neg, Add, Sub, AddAssign, SubAssign};
use num::Num;
use crate::vector::{EucVec4};

use super::Mat3;

// MAT2
#[derive(Neg, Add, Sub, AddAssign, SubAssign, Debug, PartialEq, Eq, Clone)]
pub struct Mat4<T: Num + Copy> {
    pub x: EucVec4<T>,
    pub y: EucVec4<T>,
    pub z: EucVec4<T>,
    pub w: EucVec4<T>
}

impl<T: Num + Copy> Mat4<T> {
    pub fn new (x: EucVec4<T>, y: EucVec4<T>, z: EucVec4<T>, w: EucVec4<T>) -> Mat4<T> {
        Mat4{x, y, z, w}
    }

    pub fn of (xx: T, xy: T, xz: T, xw: T, yx: T, yy: T, yz: T, yw: T, zx: T, zy: T, zz: T, zw: T, wx: T, wy: T, wz: T, ww: T) -> Mat4<T> {
        Mat4::new(
            EucVec4::new(xx, xy, xz, xw), 
            EucVec4::new(yx, yy, yz, yw),
            EucVec4::new(zx, zy, zz, zw),
            EucVec4::new(wx, wy, wz, ww)
        )
    }

    pub fn transp (&self) -> Mat4<T> {
        Mat4::of(
            self.x.x, self.y.x, self.z.x, self.w.x,
            self.x.y, self.y.y, self.z.y, self.w.y,
            self.x.z, self.y.z, self.z.z, self.w.z,
            self.x.w, self.y.w, self.z.w, self.w.w
        )
    }

    pub fn tr (&self) -> T {
        self.x.x + self.y.y + self.z.z + self.w.w
    }

    pub fn det (&self) -> T {
        todo!()
    }
}


// MULTIPLICATION
impl<T: Num + Copy> Mul<Mat4<T>> for Mat4<T> {
    type Output = Mat4<T>;

    fn mul (self, rhs: Mat4<T>) -> Self::Output {
        Mat4::of(
            self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
            self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
            self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w + self.x.w * rhs.w.w,

            self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
            self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
            self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w + self.y.w * rhs.w.w,

            self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x + self.z.w * rhs.w.x,
            self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
            self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z + self.z.w * rhs.w.z,
            self.z.x * rhs.x.w + self.z.y * rhs.y.w + self.z.z * rhs.z.w + self.z.w * rhs.w.w,

            self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x + self.w.w * rhs.w.x,
            self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y + self.w.w * rhs.w.y,
            self.w.x * rhs.x.z + self.w.y * rhs.y.z + self.w.z * rhs.z.z + self.w.w * rhs.w.z,
            self.w.x * rhs.x.w + self.w.y * rhs.y.w + self.w.z * rhs.z.w + self.w.w * rhs.w.w
        )
    }
}

impl<T: Num + Copy> Mul<EucVec4<T>> for Mat4<T> {
    type Output = EucVec4<T>;

    fn mul (self, rhs: EucVec4<T>) -> Self::Output {
        EucVec4::new(
            self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z + self.x.w * rhs.w,
            self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z + self.y.w * rhs.w,
            self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z + self.z.w * rhs.w,
            self.w.x * rhs.x + self.w.y * rhs.y + self.w.z * rhs.z + self.w.w * rhs.w
        )
    }
}

impl<T: Num + Copy> Mul<T> for Mat4<T> {
    type Output = Mat4<T>;

    fn mul (self, rhs: T) -> Self::Output {
        Mat4::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs
        )
    }
}

#[cfg(test)]
#[test]
fn mul () {
    // TODO FIX
    let alpha = Mat4::of(1, 2, 3, 10, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    assert_eq!(alpha.clone() * alpha, Mat4::of(900, 100, 110, 120, 202, 228, 254, 280, 314, 356, 398, 440, 426, 484, 542, 600))
}