use crate::traits::{Sdf, SdfGrad, SdfNoInfo};
use core::f64;
use marchrs_vectors::Vector;

/// A simple Cube:
/// - centered at `(0, 0, ...)`
/// - side length `1`
#[derive(Clone, Copy, PartialEq)]
pub struct Cuboid<const N: usize>(Vector<N>);

impl<const N: usize> Default for Cuboid<N> {
  fn default() -> Self {
    Self(Vector::ones())
  }
}

impl<const N: usize> Sdf<N> for Cuboid<N> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    let offset = pos.abs() - self.0 / 2.0;
    offset.el_max(0.0).mag() + offset.max().min(0.0)
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    self.0.iter().zip(pos).all(|(dim, x)| x <= dim / 2.0)
  }
}

impl<const N: usize> SdfNoInfo<N> for Cuboid<N> {}

impl<const N: usize> SdfGrad<N> for Cuboid<N> {}
