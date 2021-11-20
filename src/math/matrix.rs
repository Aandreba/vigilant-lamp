use std::{fmt::{Debug, Display}, ops::{Deref, Add, Sub, Mul, Div}};
use array_macro::array;
use num::Num;

use crate::{extra::{array_builder::ArrayBuilder}};
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

// TRANSPOSE
impl<T: Num + Copy, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn T (&self) -> Matrix<T, C, R> {
        let transp : [NumArray<T,R>;C] = array![i => NumArray(array![j => self[j][i]; R]); C];
        Matrix::new(transp)
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
        let array : [NumArray<T,C>;R] = array![i => self[i] + rhs[i]; R];
        Matrix::new(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Add<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add (self, rhs: T) -> Self::Output {
        let array : [NumArray<T,C>;R] = array![i => self[i] + rhs; R];
        Matrix::new(array)
    }
}

// SUBTRACTION
impl<T: Num + Copy, const R: usize, const C: usize> Sub<Matrix<T, R, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub (self, rhs: Matrix<T, R, C>) -> Self::Output {
        let array : [NumArray<T,C>;R] = array![i => self[i] - rhs[i]; R];
        Matrix::new(array)
    }
}

impl<T: Num + Copy, const R: usize, const C: usize> Sub<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub (self, rhs: T) -> Self::Output {
        let array : [NumArray<T,C>;R] = array![i => self[i] - rhs; R];
        Matrix::new(array)
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
        let array = self.0.map(|x| x * rhs);
        Matrix::new(array)
    }
}

// DIVISION
impl<T: Num + Copy, const R: usize, const C: usize> Div<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn div (self, rhs: T) -> Self::Output {
        let array : [NumArray<T,C>;R] = array![i => *(&self[i]) / rhs; R];
        Matrix::new(array)
    }
}

// PROPERTIES
impl<T: Num + Copy, const N: usize> SquareMatrix<T,N> {
    pub fn diagonal (values: [T;N]) -> SquareMatrix<T,N> {
        let array : [[T;N];N] = <[T;N]>::build2(|i| {
            let mut row = [T::zero(); N];
            row[i] = values[i];
            row
        });

        SquareMatrix::from_array(array)
    }

    pub fn identity () -> SquareMatrix<T,N> {
        let array : [[T;N];N] = <[T;N]>::build2(|i| {
            let mut row = [T::zero(); N];
            row[i] = T::one();
            row
        });

        SquareMatrix::from_array(array)
    }

    pub fn tr (&self) -> T {
        let mut sum = T::zero();
        for i in 0..N {
            sum = sum + self[i][i]
        }

        sum
    }
}

// DISPLAY
impl<T: Num + Debug, const R: usize, const C: usize> Display for Matrix<T, R, C>{
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}