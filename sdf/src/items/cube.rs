use super::Middle;
use crate::traits::{Sdf, SdfGrad, SdfNoInfo};
use core::f64;
use marchrs_vectors::Vector;
use std::array;

/// A simple Cube:
/// - centered at `(0, 0, ...)`
/// - side length `1`
#[derive(Clone, Copy, PartialEq)]
pub struct Cube<const N: usize>([Middle<N>; N]);

impl<const N: usize> Default for Cube<N> {
  fn default() -> Self {
    Self(array::from_fn(|i| Middle::new(Vector::axis(i))))
  }
}

impl<const N: usize> Sdf<N> for Cube<N> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self
      .0
      .iter()
      .map(|mid| mid.call(pos))
      .reduce(f64::max)
      .unwrap_or(f64::MIN)
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    self.0.iter().all(|mid| mid.call(pos) <= 0.0)
  }
}

impl<const N: usize> SdfNoInfo<N> for Cube<N> {}

impl<const N: usize> SdfGrad<N> for Cube<N> {}
