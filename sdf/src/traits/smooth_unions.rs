use crate::interpolate::Interpolate;

use super::{Sdf, SdfGrad, SdfInfo};
use marchrs_vectors::Vector;

/// Smooth unions for `SDF` items.\
/// heavily inspired by [this article](https://iquilezles.org/articles/smin/)
pub trait SdfSmoothUnion<const N: usize>: Sdf<N> + Sized {
  /// Creates a smooth union between to `SDF`s.
  ///
  /// By default, we use a Quadratic smooth-minimum, as it is:
  ///
  /// 1. Rigid: when objects are far enough apart, no blend is performed.
  /// 2. Conservative: never over-estimates the distance to a surface.
  /// 3. Cheap to compute.
  fn smooth_or<S: Sdf<N>>(self, other: S, factor: f64) -> SmoothUnion<Self, S> {
    SmoothUnion(self, other, factor)
  }
}

impl<const N: usize, T: Sdf<N>> SdfSmoothUnion<N> for T {}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct SmoothUnion<T, U>(pub T, pub U, pub f64);

impl<const N: usize, T: Sdf<N>, U: Sdf<N>> Sdf<N> for SmoothUnion<T, U> {
  #[inline]
  fn call(&self, pos: Vector<N>) -> f64 {
    // the modern implementation of quadratic blending,
    // faster but *doesn't* support arbitrary linear interpolation
    let v0 = self.0.call(pos);
    let v1 = self.1.call(pos);
    let f = (1.0 - (v0 - v1).abs() / (self.2 * 4.0)).max(0.0);
    v0.min(v1) - f * f * self.2
  }

  #[inline]
  fn hits(&self, pos: Vector<N>) -> bool {
    let v0 = self.0.call(pos);
    if v0 <= 0.0 {
      return true;
    }
    let v1 = self.1.call(pos);
    if v1 <= 0.0 {
      return true;
    }
    let f = (1.0 - (v0 - v1).abs() / (self.2 * 4.0)).max(0.0);
    v0.min(v1) - f * f * self.2 <= 0.0
  }
}

impl<const N: usize, T, U> SdfInfo<N> for SmoothUnion<T, U>
where
  T: SdfInfo<N>,
  U: SdfInfo<N, Info = T::Info>,
  T::Info: Interpolate,
{
  type Info = T::Info;
  #[inline]
  fn call_info(&self, pos: Vector<N>) -> (f64, Self::Info) {
    let (v0, info0) = self.0.call_info(pos);
    let (v1, info1) = self.1.call_info(pos);
    let f = (0.5 + 0.5 * (v0 - v1) / self.2).clamp(0.0, 1.0);
    (
      v0.lerp(v1, f) - self.2 * f * (1.0 - f),
      info0.lerp(info1, f),
    )
  }
}

impl<const N: usize, T: SdfGrad<N>, U: SdfGrad<N>> SdfGrad<N> for SmoothUnion<T, U> {
  /// Derived from the `call` definition:
  ///
  /// ```ignore
  /// k = self.2;
  /// v0 = self.0.call(pos);
  ///
  /// d = 0.5 + 0.5 * (v1 - v0) / k;
  /// f = clamp(d, 0, 1);
  /// r = lerp(v1, v0, f) - k * f * (1 - f);
  /// ```
  ///
  /// which has the derivative:
  ///
  /// ```ignore
  /// k = self.2;
  /// (v0, D(v0)) = self.0.call_grad(pos);
  /// (v1, D(v1)) = self.0.call_grad(pos);
  /// within(x, s, e) = f64::from((s..=e).contains(x));
  ///
  /// D(d) = 0.5 * (D(v1) - D(v0)) / k;
  /// D(f) = within(d, 0, 1) * D(d)
  /// D(r) = lerp(D(v1), D(v0), f) + D(f) * (v1 + v0 - k);
  /// ```
  ///
  /// which we return alongside the original value, allowing us to re-use code.
  #[inline]
  fn call_grad(&self, pos: Vector<N>) -> (f64, Vector<N>) {
    let (v0, grad0) = self.0.call_grad(pos);
    let (v1, grad1) = self.1.call_grad(pos);
    let k = self.2;

    let d = (v1 - v0) / (2.0 * k);
    let grad_d = (grad1 - grad0) / (2.0 * k);

    let f = d.clamp(0.0, 1.0);
    let grad_f = f64::from((0.0..=1.0).contains(&d)) * grad_d;

    let r = v1.lerp(v0, f) - k * f * (1.0 - f);
    let grad_r = grad1.lerp(grad0, f) + grad_f * (v1 + v0 - k);

    (r, grad_r)
  }
}
