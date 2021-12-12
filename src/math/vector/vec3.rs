use std::ops::{Add, Index, IndexMut, Sub, Mul, Div};
use derive_more::{AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use num::{Float, Num};

#[derive(Neg, AddAssign, SubAssign, MulAssign, DivAssign, Debug)]
pub struct EucVec3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Num> EucVec3<T> {
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
}

// VECTOR - VECTOR
impl<T: Num> Add<EucVec3<T>> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn add(self, rhs: EucVec3<T>) -> Self::Output {
       EucVec3::new(
           self.x + rhs.x,
           self.y + rhs.y,
           self.z + rhs.z
        )
    }
}

impl<T: Num> Sub<EucVec3<T>> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn sub(self, rhs: EucVec3<T>) -> Self::Output {
       EucVec3::new(
           self.x - rhs.x,
           self.y - rhs.y,
           self.z - rhs.z
        )
    }
}

impl<T: Num> Mul<EucVec3<T>> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn mul(self, rhs: EucVec3<T>) -> Self::Output {
       EucVec3::new(
           self.x * rhs.x,
           self.y * rhs.y,
           self.z * rhs.z
        )
    }
}

impl<T: Num> Div<EucVec3<T>> for EucVec3<T> {
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
impl<T: Num> Add<T> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn add(self, rhs: T) -> Self::Output {
       EucVec3::new(
           self.x + rhs,
           self.y + rhs,
           self.z + rhs
        )
    }
}

impl<T: Num> Sub<T> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
       EucVec3::new(
           self.x - rhs,
           self.y - rhs,
           self.z - rhs
        )
    }
}

impl<T: Num> Mul<T> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
       EucVec3::new(
           self.x * rhs,
           self.y * rhs,
           self.z * rhs
        )
    }
}

impl<T: Num> Div<T> for EucVec3<T> {
    type Output = EucVec3<T>;

    fn div(self, rhs: T) -> Self::Output {
       EucVec3::new(
           self.x / rhs,
           self.y / rhs,
           self.z / rhs
        )
    }
}

impl<T: Num> Index<char> for EucVec3<T> {
    type Output = T;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' => &self.x,
            'y' => &self.y,
            'z' => &self.z,
            _ => panic!("Invalid index")
        }
    }
}

impl<T: Num> IndexMut<char> for EucVec3<T> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'x' => &mut self.x,
            'y' => &mut self.y,
            'z' => &mut self.z,
            _ => panic!("Invalid index")
        }
    }
}