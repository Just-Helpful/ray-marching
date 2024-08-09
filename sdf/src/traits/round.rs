use super::{Sdf, SdfGrad, SdfInfo};
use marchrs_vectors::Vector;

pub trait SdfRound<const N: usize>: Sdf<N> + Sized {
  fn round(self, radius: f64) -> Round<N, Self> {
    Round(self, radius)
  }
}

impl<const N: usize, T: Sdf<N>> SdfRound<N> for T {}

#[derive(Clone, Copy, PartialEq)]
pub struct Round<const N: usize, T>(T, f64);

impl<const N: usize, T: Default> Default for Round<N, T> {
  fn default() -> Self {
    Self(T::default(), 0.0)
  }
}

impl<const N: usize, T: Sdf<N>> Sdf<N> for Round<N, T> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.call(pos) - self.1
  }
}

impl<const N: usize, T: SdfInfo<N>> SdfInfo<N> for Round<N, T> {
  type Info = T::Info;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    let (value, info) = self.0.call_info(pos);
    (value - self.1, info)
  }
}

impl<const N: usize, T: SdfGrad<N>> SdfGrad<N> for Round<N, T> {
  #[inline]
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    let (value, grad) = self.0.call_grad(pos);
    (value - self.1, grad)
  }
}
