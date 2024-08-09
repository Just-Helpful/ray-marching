use super::{Sdf, SdfGrad, SdfInfo};
use marchrs_vectors::Vector;

pub trait SdfTranslate<const N: usize>: Sdf<N> + Sized {
  fn translate(self, translation: impl Into<Vector<N>>) -> Translate<N, Self> {
    Translate(self, translation.into())
  }
}

impl<const N: usize, T: Sdf<N>> SdfTranslate<N> for T {}

#[derive(Clone, Copy, PartialEq)]
pub struct Translate<const N: usize, T>(T, Vector<N>);

impl<const N: usize, T: Default> Default for Translate<N, T> {
  fn default() -> Self {
    Self(T::default(), Vector::zeros())
  }
}

impl<const N: usize, T: Sdf<N>> Sdf<N> for Translate<N, T> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.call(pos - self.1)
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    self.0.hits(pos - self.1)
  }
}

impl<const N: usize, T: SdfInfo<N>> SdfInfo<N> for Translate<N, T> {
  type Info = T::Info;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    self.0.call_info(pos - self.1)
  }
}

impl<const N: usize, T: SdfGrad<N>> SdfGrad<N> for Translate<N, T> {
  #[inline]
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    self.0.call_grad(pos - self.1)
  }
}
