use num::Float;
use wasm_bindgen::{describe::WasmDescribe, convert::{IntoWasmAbi, WasmSlice}, JsValue};
use std::{fmt::{Display, Formatter, Result}, ops::{Add, Sub, Mul, Div}};

use crate::{extra::num_two::NumericTwo, math::array_ext::{NumArray, NumericArrayTraits}};
use super::matrix::{Matrix3, Matrix4};

pub type Quaternion32 = Quaternion<f32>;
pub type Quaternion64 = Quaternion<f64>;

pub struct Quaternion<T: Float> {
    pub w: T,
    pub i: T,
    pub j: T,
    pub k: T
}

// INITS
impl<T: Float> Quaternion<T> {
    pub fn new (w: T, i: T, j: T, k: T) -> Quaternion<T> {
        Quaternion {w, i, j, k}
    }

    pub fn from_array (w: T, v: [T; 3]) -> Quaternion<T> {
        Quaternion { w, i: v[0], j: v[1], k: v[2] }
    }

    pub fn from_narray (w: T, v: NumArray<T,3>) -> Quaternion<T> {
        Quaternion { w, i: v[0], j: v[1], k: v[2] }
    }

    pub fn from_angles (roll: T, pitch: T, yaw: T) -> Quaternion<T> where T: NumericTwo {
        let two = T::two();

        let rsc = (roll / two).sin_cos();
        let psc = (pitch / two).sin_cos();
        let ysc = (yaw / two).sin_cos();
        
        Quaternion::new(rsc.1 * psc.1 * ysc.1 + rsc.0 * psc.0 * ysc.0,
                        rsc.0 * psc.1 * ysc.1 - rsc.1 * psc.0 * ysc.0,
                        rsc.1 * psc.0 * ysc.1 + rsc.0 * psc.1 * ysc.0,
                        rsc.1 * psc.1 * ysc.0 - rsc.0 * psc.0 * ysc.1)
    }

    pub fn zero_rotation () -> Quaternion<T> {
        Quaternion { w: T::one(), i: T::zero(), j: T::zero(), k: T::zero() }
    }
}

// PROPERTIES
impl<T: Float> Quaternion<T> {
    pub fn conj (&self) -> Quaternion<T> {
        Quaternion::new(self.w, -self.i, -self.j, -self.k)
    }

    pub fn inverse (&self) -> Quaternion<T> {
        self.conj() / self.norm2()
    }

    pub fn norm2 (&self) -> T {
        self.w * self.w + self.i * self.i + self.j * self.j + self.k * self.k
    }

    pub fn norm (&self) -> T {
        self.norm2().sqrt()
    }

    pub fn unit (self) -> Quaternion<T> {
        self / self.norm()
    }

    pub fn vector (&self) -> NumArray<T,3> {
        NumArray([self.i, self.j, self.k])
    }

    pub fn sqrt (&self) -> Quaternion<T> where T: NumericTwo {
        let two = T::two();
        let norm = self.norm();
        let vec = self.vector();

        let alpha = ((norm + self.w) / two).sqrt();
        let beta = ((norm - self.w) / two).sqrt();

        Quaternion::from_narray(alpha, vec.unit() * beta)
    }

    pub fn exp (&self) -> Quaternion<T> {
        let exp = self.w.exp();
        let vec = self.vector();
        let norm = vec.norm();

        Quaternion::from_narray(exp * norm.cos(), (vec / norm) * norm.sin())
    }

    pub fn ln (&self) -> Quaternion<T> {
        let norm = self.norm();
        Quaternion::from_narray(norm.ln(), self.vector().unit() * (self.w / norm).acos())
    }

    pub fn rot_matrix (&self) -> Matrix3<T> where T: NumericTwo {
        let one = T::one();
        let r2 = self.w * self.w;
        let i2 = self.i * self.i;
        let j2 = self.j * self.j;
        let k2 = self.k * self.k;
        
        let ir = self.i * self.w;
        let ij = self.i * self.j;
        let ik = self.i * self.k;

        let jr = self.j * self.w;
        let jk = self.j * self.k;
        let kr = self.k * self.w;

        let s = T::two() / (r2 + i2 + j2 + k2);

        Matrix3::new([
            NumArray([one - s * (j2 + k2), s * (ij - kr), s * (ik + jr)]),
            NumArray([s * (ij + kr), one - s * (i2 + k2), s * (jk - ir)]),
            NumArray([s * (ik - jr), s * (jk + ir), one - s * (i2 + j2)])
        ])
    }

    pub fn point_rot_matrix (&self) -> Matrix3<T> where T: NumericTwo {
        let one = T::one();
        let two = T::two();

        let i2 = two * self.i * self.i;
        let j2 = two * self.j * self.j;
        let k2 = two * self.k * self.k;
        
        let ir = two * self.i * self.w;
        let ij = two * self.i * self.j;
        let ik = two * self.i * self.k;

        let jr = two * self.j * self.w;
        let jk = two * self.j * self.k;
        let kr = two * self.k * self.w;

        Matrix3::new([
            NumArray([one - k2 - j2, ij - kr, jr + ik]),
            NumArray([ij + kr, one - k2 - i2, jk - ir]),
            NumArray([ik - jr, jk + ir, one - j2 - i2])
        ])
    }

    pub fn rot_matrix4 (&self) -> Matrix4<T> where T: NumericTwo {
        let zero = T::zero();
        let one = T::one();

        let r2 = self.w * self.w;
        let i2 = self.i * self.i;
        let j2 = self.j * self.j;
        let k2 = self.k * self.k;
        
        let ir = self.i * self.w;
        let ij = self.i * self.j;
        let ik = self.i * self.k;

        let jr = self.j * self.w;
        let jk = self.j * self.k;
        let kr = self.k * self.w;

        let s = T::two() / (r2 + i2 + j2 + k2);

        Matrix4::new([
            NumArray([one - s * (j2 + k2), s * (ij - kr), s * (ik + jr), zero]),
            NumArray([s * (ij + kr), one - s * (i2 + k2), s * (jk - ir), zero]),
            NumArray([s * (ik - jr), s * (jk + ir), one - s * (i2 + j2), zero]),
            NumArray([zero, zero, zero, one])
        ])
    }

    pub fn point_rot_matrix4 (&self) -> Matrix4<T> where T: NumericTwo {
        let zero = T::zero();
        let one = T::one();
        let two = T::two();

        let i2 = two * self.i * self.i;
        let j2 = two * self.j * self.j;
        let k2 = two * self.k * self.k;
        
        let ir = two * self.i * self.w;
        let ij = two * self.i * self.j;
        let ik = two * self.i * self.k;

        let jr = two * self.j * self.w;
        let jk = two * self.j * self.k;
        let kr = two * self.k * self.w;

        Matrix4::new([
            NumArray([one - k2 - j2, ij - kr, jr + ik, zero]),
            NumArray([ij + kr, one - k2 - i2, jk - ir, zero]),
            NumArray([ik - jr, jk + ir, one - j2 - i2, zero]),
            NumArray([zero, zero, zero, one])
        ])
    }
}

// ADDITION
impl<T: Float> Add<Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn add(self, rhs: Quaternion<T>) -> Quaternion<<T as Add>::Output> {
        Quaternion::new(
            self.w + rhs.w,
            self.i + rhs.i,
            self.j + rhs.j,
            self.k + rhs.k
        )
    }
}

impl<T: Float> Add<T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn add (self, rhs: T) -> Quaternion<<T as Add>::Output> {
        Quaternion::new(
            self.w + rhs,
            self.i,
            self.j,
            self.k
        )
    }
}

// SUBTRACTION
impl<T: Float> Sub<Quaternion<T>> for Quaternion<T> {
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

impl<T: Float> Sub<T> for Quaternion<T> {
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
impl<T: Float> Mul<Quaternion<T>> for Quaternion<T> {
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

impl<T: Float> Mul<T> for Quaternion<T> {
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
impl<T: Float> Div<Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn div (self, rhs: Quaternion<T>) -> Quaternion<T> {
        self * rhs.inverse()
    }
}

impl<T: Float> Div<T> for Quaternion<T> {
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

// DISPLAY
impl<T: Float + Display> Display for Quaternion<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let zero = T::zero();
        let sign_i = if self.i >= zero { "+" } else { "-" };
        let sign_j = if self.j >= zero { "+" } else { "-" };
        let sign_k = if self.k >= zero { "+" } else { "-" };

        write!(f, "{} {} {}i {} {}j {} {}k", self.w, sign_i, self.i.abs(), sign_j, self.j.abs(), sign_k, self.k.abs())
    }
}

// CLONE
impl<T: Float> Clone for Quaternion<T> {
    fn clone(&self) -> Quaternion<T> {
        Quaternion::new(self.w, self.i, self.j, self.k)
    }
}

impl<T: Float> Copy for Quaternion<T> {}