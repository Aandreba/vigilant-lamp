use crate::extra::array_builder::ArrayBuilder;
use std::{fmt::{Debug, Display}, ops::{Deref, Add, Sub, Mul, Div}};
use num::{Num, Float};

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
        self.norm2().sqrt()
    }
}

// NUMERIC ARRAY ARITHMETIC EXTENSION
#[derive(Clone)]
pub struct NumArray<T: Num, const N: usize>(pub [T;N]);

impl<T: Num + Copy, const N: usize> NumArray<T,N>  {
    pub fn zero () -> NumArray<T,N> {
        NumArray([T::zero(); N])
    }

    pub fn one () -> NumArray<T,N> {
        NumArray([T::one(); N])
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

impl<T: Copy + Num, const N: usize> Copy for NumArray<T,N> {}