use crate::traits::{Sdf, SdfGrad, SdfNoInfo};
use marchrs_vectors::Vector;

/// A simple Plane:
/// - located at `(0, 0, ...)`
/// - facing towards `(0, ..., 1)`
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Plane<const N: usize>;

impl<const N: usize> Sdf<N> for Plane<N> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    pos[N - 1]
  }
}

impl<const N: usize> SdfNoInfo<N> for Plane<N> {}

impl<const N: usize> SdfGrad<N> for Plane<N> {
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    (self.call(pos), Vector::axis(N - 1))
  }
}

/// A complex Plane:
/// - facing towards `normal`
/// - located at `pos`
#[derive(Clone, Copy)]
pub struct GenericPlane<const N: usize>(pub Vector<N>, pub f64);

impl<const N: usize> Default for GenericPlane<N> {
  fn default() -> Self {
    Self(Vector::axis(N - 1), 0.0)
  }
}

impl<const N: usize> GenericPlane<N> {
  pub fn new(normal: impl Into<Vector<N>>, pos: impl Into<Vector<N>>) -> Self {
    let normal = normal.into().normal();
    Self(normal, normal.dot(pos.into()))
  }
}

impl<const N: usize> Sdf<N> for GenericPlane<N> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.dot(pos) - self.1
  }
}

impl<const N: usize> SdfNoInfo<N> for GenericPlane<N> {}

impl<const N: usize> SdfGrad<N> for GenericPlane<N> {
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    (self.call(pos), self.0)
  }
}
