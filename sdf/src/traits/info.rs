use super::{Sdf, SdfGrad, SdfInfo};
use marchrs_vectors::Vector;
use std::marker::PhantomData;

pub trait SdfWithInfo<const N: usize>: Sdf<N> + Sized {
  fn with<I>(self, info: impl Into<I>) -> WithInfo<Self, I> {
    WithInfo(self, info.into())
  }
}

impl<const N: usize, T: Sdf<N>> SdfWithInfo<N> for T {}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct WithInfo<T, I>(T, I);

impl<const N: usize, I, T: Sdf<N>> Sdf<N> for WithInfo<T, I> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.call(pos)
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    self.0.hits(pos)
  }
}

impl<const N: usize, I: Clone, T: Sdf<N>> SdfInfo<N> for WithInfo<T, I> {
  type Info = I;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    (self.0.call(pos), self.1.clone())
  }

  #[inline]
  fn info(&self, _: Vector<N>) -> Self::Info {
    self.1.clone()
  }
}

impl<const N: usize, I, T: SdfGrad<N>> SdfGrad<N> for WithInfo<T, I> {
  #[inline]
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    self.0.call_grad(pos)
  }
}

pub trait SdfWithDefault<const N: usize>: Sdf<N> + Sized {
  fn with_default<I>(self) -> WithDefault<Self, I> {
    WithDefault(self, PhantomData)
  }
}

impl<const N: usize, T: Sdf<N>> SdfWithDefault<N> for T {}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct WithDefault<T, I>(T, PhantomData<I>);

impl<const N: usize, I, T: Sdf<N>> Sdf<N> for WithDefault<T, I> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    self.0.call(pos)
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    self.0.hits(pos)
  }
}

impl<const N: usize, I: Default, T: Sdf<N>> SdfInfo<N> for WithDefault<T, I> {
  type Info = I;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    (self.0.call(pos), I::default())
  }

  #[inline]
  fn info(&self, _: Vector<N>) -> Self::Info {
    I::default()
  }
}

impl<const N: usize, I, T: SdfGrad<N>> SdfGrad<N> for WithDefault<T, I> {
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    self.0.call_grad(pos)
  }
}
