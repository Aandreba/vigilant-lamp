use std::ops::Mul;
use derive_more::{Neg, Add, Sub, AddAssign, SubAssign};
use num::Num;
use crate::vector::{EucVec3};

// MAT2
#[derive(Neg, Add, Sub, AddAssign, SubAssign, Debug, PartialEq, Eq, Clone)]
pub struct Mat3<T: Num + Copy> {
    pub x: EucVec3<T>,
    pub y: EucVec3<T>,
    pub z: EucVec3<T>
}

pub type Matu3 = Mat3<u64>;
pub type Mati3 = Mat3<i64>;
pub type Matf3 = Mat3<f32>;
pub type Matd3 = Mat3<f64>;

impl<T: Num + Copy> Mat3<T> {
    pub fn new (x: EucVec3<T>, y: EucVec3<T>, z: EucVec3<T>) -> Mat3<T> {
        Mat3{x, y, z}
    }

    pub fn of (xx: T, xy: T, xz: T, yx: T, yy: T, yz: T, zx: T, zy: T, zz: T) -> Mat3<T> {
        Mat3::new(
            EucVec3::new(xx, xy, xz), 
            EucVec3::new(yx, yy, yz),
            EucVec3::new(zx, zy, zz)
        )
    }

    pub fn transp (&self) -> Mat3<T> {
        Mat3::of(
            self.x.x, self.y.x, self.z.x,
            self.x.y, self.y.y, self.z.y,
            self.x.z, self.y.z, self.z.z
        )
    }

    pub fn tr (&self) -> T {
        self.x.x + self.y.y + self.z.z
    }

    pub fn det (&self) -> T {
        self.x.x * (self.y.y * self.z.z - self.y.z * self.z.y)
        - self.x.y * (self.x.x * self.z.z - self.x.z * self.z.x)
        + self.x.z * (self.x.x * self.y.y - self.x.y * self.y.x)
    }
}


// MULTIPLICATION
impl<T: Num + Copy> Mul<Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn mul (self, rhs: Mat3<T>) -> Self::Output {
        Mat3::of(
            self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
            self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z,

            self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
            self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z,

            self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x,
            self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y,
            self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z,
        )
    }
}

impl<T: Num + Copy> Mul<EucVec3<T>> for Mat3<T> {
    type Output = EucVec3<T>;

    fn mul (self, rhs: EucVec3<T>) -> Self::Output {
        EucVec3::new(
            self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z,
            self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z,
            self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z
        )
    }
}

impl<T: Num + Copy> Mul<T> for Mat3<T> {
    type Output = Mat3<T>;

    fn mul (self, rhs: T) -> Self::Output {
        Mat3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs
        )
    }
}

// OTHER TRAITS
impl<T: Num + Copy + Default> Default for Mat3<T> {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), z: Default::default() }
    }
}

#[cfg(test)]
#[test]
fn mul () {
    let alpha = Mat3::of(1, 2, 3, 4, 5, 6, 7, 8, 9);
    let beta = Mat3::of(9, 8, 7, 6, 5, 4, 3, 2, 1);
    assert_eq!(alpha * beta, Mat3::of(30, 24, 18, 84, 69, 54, 138, 114, 90))
}