use std::array;

mod dyn_wrap;
pub use dyn_wrap::*;
mod helpers;
pub use helpers::*;
mod info;
pub use info::*;
mod intersect;
pub use intersect::*;
mod invert;
pub use invert::*;
mod remove;
pub use remove::*;
mod rotate2d;
pub use rotate2d::*;
mod rotate3d;
pub use rotate3d::*;
mod round;
pub use round::*;
mod scale;
pub use scale::*;
mod smooth_unions;
pub use smooth_unions::*;
mod translate;
pub use translate::*;
mod unions;
pub use unions::*;

use marchrs_vectors::Vector;

/// A small step size, used to approximate derivatives
const EPSILON: f64 = 1e-8;

/// Objects implemented with SDFs (Signed Distance Functions),
/// where the surface is defined by `f([x, y, ...]) == 0`
///
/// @note this would be a lot more ergonomic as a `Fn([f64; N]) -> f64`<br>
/// but implementing function traits in rust isn't stable yet.<br>
/// see [#29625](https://github.com/rust-lang/rust/issues/29625)
pub trait Sdf<const N: usize> {
  /// Finds the distance to the nearest surface.<br>
  /// This can be negative if `pos` is within the item.
  fn call(&self, pos: Vector<N>) -> f64;

  /// Tests whether a specific point is inside or on an item.<br>
  /// This is implemented seperately as some items can short-circuit.
  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    self.call(pos) <= 0.0
  }
}

impl<const N: usize, F: Fn([f64; N]) -> f64 + Send + Sync> Sdf<N> for F {
  fn call(&self, pos: Vector<N>) -> f64 {
    self(pos.0)
  }
}

/// Extension to the `Sdf` that fetches the info for the closest item.
///
/// We also return the distance to the closest item, to avoid<br>
/// issues where intersection and union would have to fetch this<br>
/// distance every time the info is fetched from them, leading to<br>
/// significant recomputation in `Sdf`s with a lot of unions.
pub trait SdfInfo<const N: usize>: Sdf<N> {
  /// Information type attached to an `SDF` item.
  type Info;

  /// Calls the `SDF` and returns the info attached to the nearest item.
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info);

  /// The information attached to the nearest item.
  #[inline]
  fn info(&self, pos: Vector<N>) -> Self::Info {
    self.call_info(pos).1
  }
}

/// Extension to the `Sdf` that calculates the gradient.<br>
/// i.e. the step direction that maximises the `call` value.
///
/// ## Notes
///
/// We also return the distance to the closest item, to avoid<br>
/// issues where intersection and union would have to fetch this<br>
/// distance every time the gradient is calulated, leading to<br>
/// significant recomputation in `Sdf`s with a lot of unions.
///
/// By default, we use a numerical approximation, but this should<br>
/// be overridden when an analytical method is possible, as these<br>
/// can require significantly fewer than the `2N + 1` calls of the<br>
/// numeric method.
///
/// Whilst this signature is close enough to the `info` signature that<br>
/// it looks like `info` could be reused for gradients, the `Invert`<br>
/// operator poses a problem as it should also invert the gradient.<br>
/// However the `info` implementation on `Invert` simply passes the<br>
/// interior information upwards.<br>
///
/// Honestly, if you can think of a good workaround for this, I'd be<br>
/// more than happy to implement it, as it would significantly smooth<br>
/// out the implementation of `Sdf`s and could help prevent potential<br>
/// feature creep of this trait in the future.
pub trait SdfGrad<const N: usize>: Sdf<N> {
  /// Calls the `SDF` and fetches the gradient at the given position.
  ///
  /// Uses a numerical approximation by default.
  #[inline]
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    (
      self.call(pos),
      Vector(array::from_fn(|i| {
        let mut pos = pos;
        pos[i] -= EPSILON * 0.5;
        let value = self.call(pos);
        pos[i] += EPSILON;
        self.call(pos) - value
      }))
      .normal(),
    )
  }

  /// The gradient at a given position.
  #[inline]
  fn grad(&self, pos: Vector<N>) -> Vector<N> {
    self.call_grad(pos).1
  }
}

impl<const N: usize, F: Fn([f64; N]) -> f64 + Send + Sync> SdfGrad<N> for F {}

/// Marker trait for complete Sdf implementations
pub trait SdfFull<const N: usize>: SdfInfo<N> + SdfGrad<N> {}

impl<const N: usize, S: Sdf<N> + SdfInfo<N> + SdfGrad<N>> SdfFull<N> for S {}
