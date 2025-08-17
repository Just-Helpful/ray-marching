use crate::traits::{Sdf, SdfGrad, SdfNoInfo};
use marchrs_vectors::Vector;

/// A simple Sphere:
/// - located at `(0, 0, ...)`
/// - diameter `1`
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Sphere<const N: usize>;

impl<const N: usize> Sdf<N> for Sphere<N> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    pos.mag() - 0.5
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    pos.mag2() <= 0.25
  }
}

impl<const N: usize> SdfNoInfo<N> for Sphere<N> {}

impl<const N: usize> SdfGrad<N> for Sphere<N> {
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    (self.call(pos), pos.normal())
  }
}
