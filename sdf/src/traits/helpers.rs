use super::{Sdf, SdfInfo};
use marchrs_vectors::Vector;

/// A simple implementation of `SdfInfo` for the unit type `()`
pub trait SdfNoInfo<const N: usize>: Sdf<N> {}

impl<const N: usize, T: SdfNoInfo<N>> SdfInfo<N> for T {
  type Info = ();
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    (self.call(pos), ())
  }
}
