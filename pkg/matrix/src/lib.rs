//! Manipulations and data types that represent 2d matrix.

#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(macro_rules)]

extern crate num;

use num::{One, Zero};

/// 2D matrix.
#[deriving(PartialEq, Eq, Clone, Show)]
pub struct Matrix<T> {
    row: uint,
    column: uint,
    data: Vec<T>
}

impl<T> Matrix<T> {
    /// Creates a new `Matrix`.
    #[inline]
    pub fn from_fn(row: uint, column: uint, f: |uint, uint| -> T) -> Matrix<T> {
        Matrix {
            row: row,
            column: column,
            data: Vec::from_fn(row * column, |i| f(i / column, i % column))
        }
    }

    /// Creates a new `Matrix` from vector.
    #[inline]
    pub fn from_vec(row: uint, column: uint, data: Vec<T>) -> Matrix<T> {
        assert_eq!(row * column, data.len());
        Matrix { row: row, column: column, data: data }
    }

    /// Returns the matrix's row and column.
    #[inline]
    pub fn size(&self) -> (uint, uint) { (self.row(), self.column()) }
    /// Returns the matrix's row.
    #[inline]
    pub fn row(&self) -> uint { self.row }
    /// Returns the matrix's column.
    #[inline]
    pub fn column(&self) -> uint { self.column }
}

impl<T: Zero> Matrix<T> {
    /// Creates a matrix whose elements are all zero.
    #[inline]
    pub fn zero(row: uint, column: uint) -> Matrix<T> {
        Matrix::from_fn(row, column, |_, _| Zero::zero())
    }
}

impl<T: One + Zero> Matrix<T> {
    /// Creates a identity matrix.
    #[inline]
    pub fn one(row: uint, column: uint) -> Matrix<T> {
        Matrix::from_fn(row, column, |i, j| {
            if i == j {
                One::one()
            } else {
                Zero::zero()
            }
        })
    }
}

impl<T: Clone> Matrix<T> {
    #[inline]
    /// Returns transpose of the matrix.
    pub fn trans(&self) -> Matrix<T> {
        Matrix::from_fn(self.column(), self.row(), |i, j| self[(j, i)].clone())
    }
}

impl<T> Index<(uint, uint), T> for Matrix<T> {
    #[inline]
    fn index(&self, &(i, j): &(uint, uint)) -> &T {
        assert!(i < self.row() && j < self.column());
        &self.data[i * self.column() + j]
    }
}

macro_rules! forward_val_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<Lhs, Rhs, Result> $imp<Matrix<Rhs>, Matrix<Result>> for Matrix<Lhs>
            where Lhs: $imp<Rhs, Result> + Clone, Rhs: Clone {
            #[inline]
            fn $method(self, other: Matrix<Rhs>) -> Matrix<Result> {
                (&self).$method(&other)
            }
        }
    }
}

macro_rules! forward_ref_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, Lhs, Rhs, Result> $imp<Matrix<Rhs>, Matrix<Result>> for &'a Matrix<Lhs>
            where Lhs: $imp<Rhs, Result> + Clone, Rhs: Clone {
            #[inline]
            fn $method(self, other: Matrix<Rhs>) -> Matrix<Result> {
                self.$method(&other)
            }
        }
    }
}

macro_rules! forward_val_ref_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, Lhs, Rhs, Result> $imp<&'a Matrix<Rhs>, Matrix<Result>> for Matrix<Lhs>
            where Lhs: $imp<Rhs, Result> + Clone, Rhs: Clone {
            #[inline]
            fn $method(self, other: &Matrix<Rhs>) -> Matrix<Result> {
                (&self).$method(other)
            }
        }
    }
}

macro_rules! forward_all_binop {
    (impl $imp:ident, $method:ident) => {
        forward_val_val_binop!(impl $imp, $method);
        forward_ref_val_binop!(impl $imp, $method);
        forward_val_ref_binop!(impl $imp, $method);
    };
}

forward_all_binop!(impl Add, add);

impl<'a, 'b, Lhs, Rhs, Result> Add<&'b Matrix<Rhs>, Matrix<Result>> for &'a Matrix<Lhs>
    where Lhs: Add<Rhs, Result> + Clone, Rhs: Clone {
    #[inline]
    fn add(self, other: &Matrix<Rhs>) -> Matrix<Result> {
        assert_eq!(self.size(), other.size());
        Matrix::from_fn(self.row(), self.column(), |i, j| self[(i, j)].clone() + other[(i, j)].clone())
    }
}

forward_all_binop!(impl Sub, sub);

impl<'a, 'b, Lhs, Rhs, Result> Sub<&'b Matrix<Rhs>, Matrix<Result>> for &'a Matrix<Lhs>
    where Lhs: Sub<Rhs, Result> + Clone, Rhs: Clone {
    #[inline]
    fn sub(self, other: &Matrix<Rhs>) -> Matrix<Result> {
        assert_eq!(self.size(), other.size());
        Matrix::from_fn(self.row(), self.column(), |i, j| self[(i, j)].clone() - other[(i, j)].clone())
    }
}

impl<Lhs, Rhs, Result> Mul<Matrix<Rhs>, Matrix<Result>> for Matrix<Lhs>
    where Lhs: Mul<Rhs, Result> + Clone, Rhs: Clone, Result: Add<Result, Result> {
    #[inline]
    fn mul(self, other: Matrix<Rhs>) -> Matrix<Result> { (&self).mul(&other) }
}

impl<'a, Lhs, Rhs, Result> Mul<Matrix<Rhs>, Matrix<Result>> for &'a Matrix<Lhs>
    where Lhs: Mul<Rhs, Result> + Clone, Rhs: Clone, Result: Add<Result, Result> {
    #[inline]
    fn mul(self, other: Matrix<Rhs>) -> Matrix<Result> { self.mul(&other) }
}

impl<'a, Lhs, Rhs, Result> Mul<&'a Matrix<Rhs>, Matrix<Result>> for Matrix<Lhs>
    where Lhs: Mul<Rhs, Result> + Clone, Rhs: Clone, Result: Add<Result, Result> {
    #[inline]
    fn mul(self, other: &Matrix<Rhs>) -> Matrix<Result> { self.mul(other) }
}

impl<'a, 'b, Lhs, Rhs, Result> Mul<&'b Matrix<Rhs>, Matrix<Result>> for &'a Matrix<Lhs>
    where Lhs: Mul<Rhs, Result> + Clone, Rhs: Clone, Result: Add<Result, Result> {
    #[inline]
    fn mul(self, other: &Matrix<Rhs>) -> Matrix<Result> {
        assert_eq!(self.column(), other.row());
        Matrix::from_fn(self.row(), other.column(), |i, j| {
            let mut sum = self[(i, 0)].clone() * other[(0, j)].clone();
            for k in range(1, self.column()) {
                sum = sum + self[(i, k)].clone() * other[(k, j)].clone();
            }
            sum
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn from_vec() {
        let mat = Matrix::from_vec(2, 3, vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]);
        for i in range(0, mat.row()) {
            for j in range(0, mat.column()) {
                assert_eq!((i, j), mat[(i, j)]);
            }
        }
    }

    #[test]
    fn index() {
        let mat = Matrix::from_fn(3, 5, |i, j| (i, j));
        for i in range(0, mat.row()) {
            for j in range(0, mat.column()) {
                assert_eq!((i, j), mat[(i, j)]);
            }
        }
    }


    #[test]
    fn mul() {
        let m1 = Matrix::from_vec(1, 3, vec![1.0f64, 2.0, 3.0]);
        let m2 = Matrix::from_vec(3, 1, vec![1.0, 2.0, 3.0]);
        assert_eq!(Matrix::from_vec(1, 1, vec![14.0]), m1 * m2);
        assert_eq!(Matrix::from_vec(3, 1, vec![1.0f64, 4.0, 7.0]),
                   Matrix::from_vec(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]) *
                   Matrix::from_vec(3, 1, vec![1.0, 0.0, 0.0]));
        assert_eq!(Matrix::from_vec(3, 2, vec![1.0f64, 3.0, 4.0, 6.0, 7.0, 9.0]),
                   Matrix::from_vec(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]) *
                   Matrix::from_vec(3, 2, vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
    }
}

