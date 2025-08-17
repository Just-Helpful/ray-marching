use super::{Sdf, SdfGrad, SdfInfo};
use marchrs_vectors::Vector;

pub trait SdfScale<const N: usize>: Sdf<N> + Sized {
  fn scale(self, scale: impl Into<Vector<N>>) -> Scale<N, Self> {
    let scale = scale.into();
    Scale(self, 1.0 / scale, scale.min())
  }
}

impl<const N: usize, T: Sdf<N>> SdfScale<N> for T {}

/// A Scaled `SDF` item `Scale(item, iscale, min)`
/// - `item`: the `SDF` item to apply scaling to
/// - `iscale`: `1.0 / scale`, precomputed for speed
/// - `min`: the minimum axis of `scale`, precomputed for speed
#[derive(Clone, Copy, PartialEq)]
pub struct Scale<const N: usize, T>(pub T, Vector<N>, f64);

impl<const N: usize, T: Default> Default for Scale<N, T> {
  fn default() -> Self {
    Self(T::default(), Vector([1.0; N]), 1.0)
  }
}

impl<const N: usize, T: Sdf<N>> Sdf<N> for Scale<N, T> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.call(pos * self.1) * self.2
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    self.0.hits(pos * self.1)
  }
}

impl<const N: usize, T: SdfInfo<N>> SdfInfo<N> for Scale<N, T> {
  type Info = T::Info;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    self.0.call_info(pos * self.1)
  }
}

impl<const N: usize, T: SdfGrad<N>> SdfGrad<N> for Scale<N, T> {
  #[inline]
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    self.0.call_grad(pos * self.1)
  }
}
