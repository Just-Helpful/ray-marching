use std::rc::Rc;

use super::{Sdf, SdfFull, SdfGrad, SdfInfo};
use marchrs_vectors::Vector;

pub trait SdfDynWrap<const N: usize>: SdfFull<N> + Sized + 'static {
  /// Wraps the model in an `Rc<dyn ...>` to make typing easier.
  ///
  /// Effectively, this erases all the compound types from a SDF model,<br>
  /// turning `WithInfo<Scale<Sphere<3>>, bool>` into `DynWrap<3, bool>`,<br>
  /// which is significantly easier to pass around.
  ///
  /// This also sort of has the benefit of allowing `PartialEq` on things<br>
  /// that don't typically support it, i.e. closures.
  fn wrap(self) -> DynModel<N, Self::Info> {
    DynModel(Rc::new(self))
  }
}

impl<const N: usize, T: SdfFull<N> + 'static> SdfDynWrap<N> for T {}

/// A model in a dynamic `Rc<dyn ...>` wrapper.
#[derive(Clone)]
pub struct DynModel<const N: usize, I>(Rc<dyn SdfFull<N, Info = I>>);

impl<const N: usize, I> PartialEq for DynModel<N, I> {
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.0, &other.0)
  }
}

impl<const N: usize, I> Sdf<N> for DynModel<N, I> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.call(pos)
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    self.0.hits(pos)
  }
}

impl<const N: usize, I> SdfInfo<N> for DynModel<N, I> {
  type Info = I;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    self.0.call_info(pos)
  }
}

impl<const N: usize, I> SdfGrad<N> for DynModel<N, I> {
  #[inline]
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    self.0.call_grad(pos)
  }
}
