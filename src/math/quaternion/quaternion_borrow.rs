use num::Float;
use std::ops::{Add, Sub, Mul, Div};
use super::quaternion::Quaternion;

// ADDITION
impl<T: Float> Add<&Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn add(self, rhs: &Quaternion<T>) -> Quaternion<<T as Add>::Output> {
        Quaternion::new(
            self.w + rhs.w,
            self.i + rhs.i,
            self.j + rhs.j,
            self.k + rhs.k
        )
    }
}

impl<T: Float> Add<Quaternion<T>> for &Quaternion<T> {
    type Output = Quaternion<T>;

    fn add(self, rhs: Quaternion<T>) -> Quaternion<T> {
        Quaternion::new(
            self.w + rhs.w,
            self.i + rhs.i,
            self.j + rhs.j,
            self.k + rhs.k
        )
    }
}

impl<T: Float> Add<&T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn add (self, rhs: &T) -> Quaternion<T> {
        Quaternion::new(
            self.w + *rhs,
            self.i,
            self.j,
            self.k
        )
    }
}

impl<T: Float> Add<T> for &Quaternion<T> {
    type Output = Quaternion<T>;

    fn add (self, rhs: T) -> Quaternion<T> {
        Quaternion::new(
            self.w + rhs,
            self.i,
            self.j,
            self.k
        )
    }
}

// SUBTRACTION
impl<T: Float> Sub<&Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn sub (self, rhs: &Quaternion<T>) -> Quaternion<T> {
        Quaternion::new(
            self.w - rhs.w,
            self.i - rhs.i,
            self.j - rhs.j,
            self.k - rhs.k
        )
    }
}

impl<T: Float> Sub<Quaternion<T>> for &Quaternion<T> {
    type Output = Quaternion<T>;

    fn sub (self, rhs: Quaternion<T>) -> Quaternion<T> {
        Quaternion::new(
            self.w - rhs.w,
            self.i - rhs.i,
            self.j - rhs.j,
            self.k - rhs.k
        )
    }
}

impl<T: Float> Sub<&T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn sub (self, rhs: &T) -> Quaternion<T> {
        Quaternion::new(
            self.w - *rhs,
            self.i,
            self.j,
            self.k
        )
    }
}

impl<T: Float> Sub<T> for &Quaternion<T> {
    type Output = Quaternion<T>;

    fn sub (self, rhs: T) -> Quaternion<T> {
        Quaternion::new(
            self.w - rhs,
            self.i,
            self.j,
            self.k
        )
    }
}

// MULTIPLICATION
impl<T: Float> Mul<&Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul (self, rhs: &Quaternion<T>) -> Quaternion<T> {
        Quaternion::new (
            self.w * rhs.w - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            self.w * rhs.i + self.i * rhs.w + self.j * rhs.k - self.k * rhs.j,
            self.w * rhs.j - self.i * rhs.k + self.j * rhs.w + self.k * rhs.i,
            self.w * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.w
        )
    }
}

impl<T: Float> Mul<Quaternion<T>> for &Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul (self, rhs: Quaternion<T>) -> Quaternion<T> {
        Quaternion::new (
            self.w * rhs.w - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            self.w * rhs.i + self.i * rhs.w + self.j * rhs.k - self.k * rhs.j,
            self.w * rhs.j - self.i * rhs.k + self.j * rhs.w + self.k * rhs.i,
            self.w * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.w
        )
    }
}

impl<T: Float> Mul<&T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul (self, rhs: &T) -> Quaternion<T> {
        Quaternion::new (
            self.w * *rhs,
            self.i * *rhs,
            self.j * *rhs,
            self.k * *rhs
        )
    }
}

impl<T: Float> Mul<T> for &Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul (self, rhs: T) -> Quaternion<T> {
        Quaternion::new (
            self.w * rhs,
            self.i * rhs,
            self.j * rhs,
            self.k * rhs
        )
    }
}

// DIVISION
impl<T: Float> Div<&T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn div (self, rhs: &T) -> Quaternion<T> {
        Quaternion::new (
            self.w / *rhs,
            self.i / *rhs,
            self.j / *rhs,
            self.k / *rhs
        )
    }
}

impl<T: Float> Div<T> for &Quaternion<T> {
    type Output = Quaternion<T>;

    fn div (self, rhs: T) -> Quaternion<T> {
        Quaternion::new (
            self.w / rhs,
            self.i / rhs,
            self.j / rhs,
            self.k / rhs
        )
    }
}