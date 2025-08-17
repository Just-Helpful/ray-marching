use super::{Sdf, SdfGrad, SdfInfo};
use marchrs_vectors::Vector;

pub trait SdfIntersect<const N: usize>: Sdf<N> + Sized {
  fn and<S: Sdf<N>>(self, other: S) -> Intersect<Self, S> {
    Intersect(self, other)
  }
}

impl<const N: usize, T: Sdf<N>> SdfIntersect<N> for T {}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Intersect<T, U>(pub T, pub U);

impl<const N: usize, T: Sdf<N>, U: Sdf<N>> Sdf<N> for Intersect<T, U> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.call(pos).max(self.1.call(pos))
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    // `&&` short circuits, so can cull function calls
    self.0.hits(pos) && self.1.hits(pos)
  }
}

impl<const N: usize, T: SdfInfo<N>, U: SdfInfo<N, Info = T::Info>> SdfInfo<N> for Intersect<T, U> {
  type Info = T::Info;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    let (value0, info0) = self.0.call_info(pos);
    let (value1, info1) = self.1.call_info(pos);
    if value0 >= value1 {
      (value0, info0)
    } else {
      (value1, info1)
    }
  }
}

impl<const N: usize, T: SdfGrad<N>, U: SdfGrad<N>> SdfGrad<N> for Intersect<T, U> {
  #[inline]
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    let (value0, grad0) = self.0.call_grad(pos);
    let (value1, grad1) = self.1.call_grad(pos);
    if value0 >= value1 {
      (value0, grad0)
    } else {
      (value1, grad1)
    }
  }
}
