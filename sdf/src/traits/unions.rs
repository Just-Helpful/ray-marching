use super::{Sdf, SdfInfo};
use marchrs_vectors::Vector;

pub trait SdfUnion<const N: usize>: Sdf<N> + Sized {
  fn or<S: Sdf<N>>(self, other: S) -> Union<Self, S> {
    Union(self, other)
  }
}

impl<const N: usize, T: Sdf<N>> SdfUnion<N> for T {}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Union<T, U>(pub T, pub U);

impl<const N: usize, T: Sdf<N>, U: Sdf<N>> Sdf<N> for Union<T, U> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.call(pos).min(self.1.call(pos))
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    // `||` short circuits, so can cull function calls
    self.0.hits(pos) || self.1.hits(pos)
  }
}

impl<const N: usize, T: SdfInfo<N>, U: SdfInfo<N, Info = T::Info>> SdfInfo<N> for Union<T, U> {
  type Info = T::Info;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    let (value0, info0) = self.0.call_info(pos);
    let (value1, info1) = self.1.call_info(pos);
    if value0 <= value1 {
      (value0, info0)
    } else {
      (value1, info1)
    }
  }
}
