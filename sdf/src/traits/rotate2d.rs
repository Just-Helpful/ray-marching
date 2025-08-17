use super::{Sdf, SdfGrad, SdfInfo};
use marchrs_vectors::Vector;

pub trait SdfRotate2D: Sdf<2> + Sized {
  fn rot(self, angle: f64) -> Rotate2D<Self> {
    let s = (-angle).sin();
    let c = (-angle).cos();
    Rotate2D(self, [[c, -s], [c, s]])
  }
}

impl<T: Sdf<2>> SdfRotate2D for T {}

#[derive(Clone, Copy, PartialEq)]
pub struct Rotate2D<T>(pub T, [[f64; 2]; 2]);

impl<T: Default> Default for Rotate2D<T> {
  fn default() -> Self {
    Self(T::default(), [[1.0, 0.0], [0.0, 1.0]])
  }
}

impl<T: Sdf<2>> Sdf<2> for Rotate2D<T> {
  #[inline]
  fn call(&self, pos: Vector<2>) -> f64 {
    self.0.call(self.1 * pos)
  }

  #[inline]
  fn hits(&self, pos: Vector<2>) -> bool {
    self.0.hits(self.1 * pos)
  }
}

impl<T: SdfInfo<2>> SdfInfo<2> for Rotate2D<T> {
  type Info = T::Info;
  #[inline]
  fn call_info(&self, pos: Vector<2>) -> (f64, Self::Info) {
    self.0.call_info(self.1 * pos)
  }
}

impl<T: SdfGrad<2>> SdfGrad<2> for Rotate2D<T> {
  #[inline]
  fn call_grad(&self, pos: Vector<2>) -> (f64, Vector<2>) {
    self.0.call_grad(self.1 * pos)
  }
}
