use crate::traits::{Sdf, SdfGrad, SdfNoInfo};
use marchrs_vectors::Vector;

/// A set of two planes `Middle(axis)`:
/// - centered on `(0, 0, ...)`
/// - along axis `axis`
/// - a distance of `1` apart
#[derive(Clone, Copy, PartialEq)]
pub struct Middle<const N: usize>(Vector<N>);

impl<const N: usize> Default for Middle<N> {
  fn default() -> Self {
    Self(Vector::axis(N - 1))
  }
}

impl<const N: usize> Middle<N> {
  pub fn new(axis: impl Into<Vector<N>>) -> Self {
    Self(axis.into().normal())
  }
}

impl<const N: usize> Sdf<N> for Middle<N> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.dot(pos).abs() - 0.5
  }
}

impl<const N: usize> SdfNoInfo<N> for Middle<N> {}

impl<const N: usize> SdfGrad<N> for Middle<N> {
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    let d = self.0.dot(pos);
    (d.abs() - 0.5, d.signum() * self.0)
  }
}
