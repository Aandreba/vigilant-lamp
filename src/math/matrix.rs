use std::{fmt::{Debug, Display}, ops::{Deref, Add, Sub, Mul, Div}};
use num::Num;

use crate::extra::array_builder::ArrayBuilder;
use super::array_ext::NumArray;

pub struct Matrix<T: Num, const R: usize, const C: usize>([NumArray<T,C>;R]);

// TYPES
pub type SquareMatrix<T, const N: usize> = Matrix<T, N, N>;
pub type Matrix4<T> = SquareMatrix<T, 4>;
pub type Matrix3<T> = SquareMatrix<T, 3>;
pub type Matrix2<T> = SquareMatrix<T, 2>;

// BASE
impl<T: Num, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new (value: [NumArray<T,C>;R]) -> Matrix<T, R, C> {
        Matrix(value)
    }

    pub fn from_array (value: [[T;C];R]) -> Matrix<T, R, C> {
        Matrix(value.map(|x| NumArray(x)))
    }

    pub fn rows () -> usize {
        R
    }

    pub fn cols () -> usize {
        C
    }
}

// DEREF
impl<T: Num, const R: usize, const C: usize> Deref for Matrix<T,R,C> {
    type Target = [NumArray<T,C>;R];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ADDITION
impl<T: Num + Copy, const R: usize, const C: usize> Add<Matrix<T, R, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add (self, rhs: Matrix<T, R, C>) -> Self::Output {
        let array : [[T;C];R] = <[T;C]>::build2(|i| <[T;C]>::build(|j| self[i][j] + rhs[i][j]));
        Matrix::from_array(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Add<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add (self, rhs: T) -> Self::Output {
        let array : [[T;C];R] = <[T;C]>::build2(|i| <[T;C]>::build(|j| self[i][j] + rhs));
        Matrix::from_array(array)
    }
}

// SUBTRACTION
impl<T: Num + Copy, const R: usize, const C: usize> Sub<Matrix<T, R, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub (self, rhs: Matrix<T, R, C>) -> Self::Output {
        let array : [[T;C];R] = <[T;C]>::build2(|i| <[T;C]>::build(|j| self[i][j] - rhs[i][j]));
        Matrix::from_array(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Sub<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub (self, rhs: T) -> Self::Output {
        let array : [[T;C];R] = <[T;C]>::build2(|i| <[T;C]>::build(|j| self[i][j] - rhs));
        Matrix::from_array(array)
    }
}

// MULTIPLICATION
impl<T: Num + Copy, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, K, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, C, K>;

    fn mul (self, rhs: Matrix<T, K, C>) -> Matrix<T, C, K> {
        let array : [[T;K];C] = <[T;K]>::build2(|i| {
            return <[T;K]>::build(|j| {
                let mut sum : T = T::zero();
                for k in 0..K {
                    sum = sum + (self[i][k] * rhs[k][j]);
                }

                return sum;
            });
        });
        
        Matrix::from_array(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn mul (self, rhs: T) -> Self::Output {
        let array : [[T;C];R] = <[T;C]>::build2(|i| <[T;C]>::build(|j| self[i][j] * rhs));
        Matrix::from_array(array)
    }
}

// DIVISION
impl<T: Num + Copy, const R: usize, const C: usize> Div<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn div (self, rhs: T) -> Self::Output {
        let array : [[T;C];R] = <[T;C]>::build2(|i| <[T;C]>::build(|j| self[i][j] / rhs));
        Matrix::from_array(array)
    }
}

// DISPLAY
impl<T: Num + Debug, const R: usize, const C: usize> Display for Matrix<T, R, C>{
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}