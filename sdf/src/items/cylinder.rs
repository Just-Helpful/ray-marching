use crate::traits::{Sdf, SdfNoInfo};
use marchrs_vectors::Vector;

/// An infinite `Cylinder(axis)`
/// - aligned with `axis`
/// - radius `0.5`
#[derive(Clone, Copy, PartialEq)]
pub struct Cylinder<const N: usize>(Vector<N>);

impl<const N: usize> Default for Cylinder<N> {
  fn default() -> Self {
    Self(Vector::axis(N - 1))
  }
}

impl<const N: usize> Cylinder<N> {
  pub fn new(axis: impl Into<Vector<N>>) -> Self {
    Self(axis.into())
  }
}

impl<const N: usize> Sdf<N> for Cylinder<N> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    (pos - (pos.dot(self.0)) * self.0).mag() - 0.5
  }
}

impl<const N: usize> SdfNoInfo<N> for Cylinder<N> {}

/// A capped `Cylinder(axis)`
/// - aligned with `axis`
/// - radius `0.5`
/// - between `[-0.5, 0.5]` on the axis
#[derive(Clone, Copy)]
pub struct CapCylinder<const N: usize>(Vector<N>);

impl<const N: usize> Default for CapCylinder<N> {
  fn default() -> Self {
    Self(Vector::axis(N - 1))
  }
}

impl<const N: usize> CapCylinder<N> {
  pub fn new(axis: impl Into<Vector<N>>) -> Self {
    Self(axis.into())
  }
}

impl<const N: usize> Sdf<N> for CapCylinder<N> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    let proj = pos.dot(self.0) * self.0;
    let perp = pos - proj;
    proj.mag().max(perp.mag()) - 0.5
  }
}

impl<const N: usize> SdfNoInfo<N> for CapCylinder<N> {}
