use core::f64;
use std::{
  array,
  ops::{Deref, DerefMut, Mul},
};

/// A very simple mathematical vector implementation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<const N: usize>(pub [f64; N]);

mod constructors;

impl<const N: usize> Deref for Vector<N> {
  type Target = [f64; N];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl<const N: usize> DerefMut for Vector<N> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
impl<const N: usize> IntoIterator for Vector<N> {
  type Item = f64;
  type IntoIter = array::IntoIter<f64, N>;
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

mod operators;

impl<const N: usize> Vector<N> {
  /// Minimum of a vector
  #[inline]
  pub fn min(self) -> f64 {
    let mut result = f64::NAN;
    for x in self {
      result = result.min(x);
    }
    result
  }

  /// Maximum of a vector
  #[inline]
  pub fn max(self) -> f64 {
    let mut result = f64::NAN;
    for x in self {
      result = result.max(x);
    }
    result
  }

  /// Element-wise minimum with value
  #[inline]
  pub fn el_min(self, rhs: impl Into<Self>) -> Self {
    let rhs = rhs.into();
    Self(array::from_fn(move |i| self[i].min(rhs[i])))
  }

  /// Element-wise maximum with value
  #[inline]
  pub fn el_max(self, rhs: impl Into<Self>) -> Self {
    let rhs = rhs.into();
    Self(array::from_fn(move |i| self[i].max(rhs[i])))
  }

  /// Element-wise absolute value
  #[inline]
  pub fn abs(self) -> Self {
    Self(self.0.map(f64::abs))
  }

  /// Magnitude of a vector
  #[inline]
  pub fn mag(self) -> f64 {
    self.iter().map(|x| x * x).sum::<f64>().sqrt()
  }

  /// Normalises a vector
  #[inline]
  pub fn normal(self) -> Self {
    self / self.mag()
  }

  /// Dot Product
  #[inline]
  pub fn dot(self, rhs: Self) -> f64 {
    self.into_iter().zip(rhs).map(|(l, r)| l * r).sum()
  }

  /// Matrix Product
  #[inline]
  pub fn matmul(self, mat: [[f64; N]; N]) -> Self {
    Self(mat.map(|row| Vector(row).dot(self)))
  }
}

pub fn transpose<const N: usize>(mat: [[f64; N]; N]) -> [[f64; N]; N] {
  array::from_fn(|i| array::from_fn(|j| mat[j][i]))
}

impl<const N: usize> Mul<Vector<N>> for [[f64; N]; N] {
  type Output = Vector<N>;
  fn mul(self, rhs: Vector<N>) -> Self::Output {
    rhs.matmul(self)
  }
}

mod iterators;
pub use iterators::GridIter;
