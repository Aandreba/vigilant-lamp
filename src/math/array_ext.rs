use crate::extra::array_builder::ArrayBuilder;
use std::{fmt::{Debug, Display}, ops::{Add, Deref, DerefMut, Div, Mul, Sub}};
use num::{Num, Float};
use wasm_bindgen::{describe::WasmDescribe, convert::{IntoWasmAbi, WasmSlice}};

// NUMERIC ARRAY EXTENSION
pub trait NumericArrayTraits<T: Copy + Num, const N: usize> {
    fn sum (&self) -> T;
    fn norm2 (&self) -> T;
    fn norm (&self) -> T where T: Float;
}

impl<T: Copy + Num, const N: usize> NumericArrayTraits<T,N> for [T;N] {
    fn sum (&self) -> T {
        let mut sum = T::zero();
        for x in self {
            sum = sum + *x;
        }

        return sum;
    }

    fn norm2 (&self) -> T {
        self.map(|f| f * f).sum()
    }

    fn norm (&self) -> T where T: Float {
        if N == 2 {
            return self[0].hypot(self[1]);
        }

        self.norm2().sqrt()
    }
}

// NUMERIC ARRAY ARITHMETIC EXTENSION
#[derive(Clone)]
pub struct NumArray<T: Num, const N: usize> (pub [T;N]);

impl<T: Num + Copy, const N: usize> NumArray<T,N>  {
    pub fn zero () -> NumArray<T,N> {
        NumArray([T::zero(); N])
    }

    pub fn one () -> NumArray<T,N> {
        NumArray([T::one(); N])
    }

    pub fn x (&self) -> T {
        self[0]
    }

    pub fn y (&self) -> T {
        self[1]
    }

    pub fn z (&self) -> T {
        self[2]
    }

    pub fn w (&self) -> T {
        self[3]
    }
}

impl<T: Copy + Float, const N: usize> NumArray<T,N> {
    pub fn unit (self) -> NumArray<T,N> {
        let norm = &self.norm();
        return self / *norm;
    }
}

impl<T: Num, const N: usize> Deref for NumArray<T,N> {
    type Target = [T;N];

    fn deref(&self) -> &[T;N] {
        &self.0
    }
}

impl<T: Num, const N: usize> DerefMut for NumArray<T,N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Num + Debug, const N: usize> Display for NumArray<T,N>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Num + Debug, const N: usize> Debug for NumArray<T,N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NumArray").field(&self.0).finish()
    }
}

// ADDITION
impl<T: Copy + Num, const N: usize> Add<NumArray<T,N>> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn add (self, rhs: NumArray<T,N>) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] + rhs[i]);
        return NumArray(array);
    }
}

impl<T: Copy + Num, const N: usize> Add<[T;N]> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn add (self, rhs: [T;N]) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] + rhs[i]);
        return NumArray(array);
    }
}

impl<T: Copy + Num, const N: usize> Add<T> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn add (self, rhs: T) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] + rhs);
        return NumArray(array);
    }
}

// SUBTRACTION
impl<T: Copy + Num, const N: usize> Sub<NumArray<T,N>> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn sub (self, rhs: NumArray<T,N>) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] - rhs[i]);
        return NumArray(array);
    }
}

impl<T: Copy + Num, const N: usize> Sub<[T;N]> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn sub (self, rhs: [T;N]) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] - rhs[i]);
        return NumArray(array);
    }
}

impl<T: Copy + Num, const N: usize> Sub<T> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn sub (self, rhs: T) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] - rhs);
        return NumArray(array);
    }
}

// MULTIPLICATION
impl<T: Copy + Num, const N: usize> Mul<NumArray<T,N>> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn mul (self, rhs: NumArray<T,N>) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] * rhs[i]);
        return NumArray(array);
    }
}

impl<T: Copy + Num, const N: usize> Mul<[T;N]> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn mul (self, rhs: [T;N]) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] * rhs[i]);
        return NumArray(array);
    }
}

impl<T: Copy + Num, const N: usize> Mul<T> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn mul (self, rhs: T) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] * rhs);
        return NumArray(array);
    }
}

// DIVISION
impl<T: Copy + Num, const N: usize> Div<NumArray<T,N>> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn div (self, rhs: NumArray<T,N>) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] / rhs[i]);
        return NumArray(array);
    }
}

impl<T: Copy + Num, const N: usize> Div<[T;N]> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn div (self, rhs: [T;N]) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] / rhs[i]);
        return NumArray(array);
    }
}

impl<T: Copy + Num, const N: usize> Div<T> for NumArray<T,N> {
    type Output = NumArray<T,N>;

    fn div (self, rhs: T) -> NumArray<T,N> {
        let array = <[T;N]>::build(|i| self[i] / rhs);
        return NumArray(array);
    }
}

impl<const N: usize> IntoWasmAbi for NumArray<f32,N> {
    type Abi = WasmSlice;

    fn into_abi (self) -> Self::Abi {
        self.0.into_abi()
    }
}

impl<T: Copy + Num, const N: usize> Copy for NumArray<T,N> {}
impl<T: Copy + Num, const N: usize> WasmDescribe for NumArray<T,N> {
    fn describe() {
    }
}