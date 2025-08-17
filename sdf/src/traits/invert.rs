use super::{Sdf, SdfGrad, SdfInfo};
use marchrs_vectors::Vector;

pub trait SdfInvert<const N: usize>: Sdf<N> + Sized {
  fn not(self) -> Invert<Self> {
    Invert(self)
  }
}

impl<const N: usize, T: Sdf<N>> SdfInvert<N> for T {}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Invert<T>(pub T);

impl<const N: usize, T: Sdf<N>> Sdf<N> for Invert<T> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    -self.0.call(pos)
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    !self.0.hits(pos)
  }
}

impl<const N: usize, T: SdfInfo<N>> SdfInfo<N> for Invert<T> {
  type Info = T::Info;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    let (value, info) = self.0.call_info(pos);
    (-value, info)
  }
}

impl<const N: usize, T: SdfGrad<N>> SdfGrad<N> for Invert<T> {
  #[inline]
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    let (value, grad) = self.0.call_grad(pos);
    (-value, -grad)
  }
}
