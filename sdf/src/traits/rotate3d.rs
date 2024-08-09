use marchrs_vectors::Vector;

use super::{Sdf, SdfGrad, SdfInfo};

/// 3D rotation around an axis.<br>
/// we just use the matrix transformation from [wikipedia](http://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle)
/// to get the inverse rotation matrix to be applied to coordinates.
pub trait SdfRotate3D: Sdf<3> + Sized {
  fn rot(self, axis: impl Into<Vector<3>>, angle: f64) -> Rotate3D<Self> {
    let s = (-angle).sin();
    let c = (-angle).cos();
    let Vector([x, y, z]) = axis.into().normal();
    Rotate3D(
      self,
      [
        [
          c + x * x * (1.0 - c),
          x * y * (1.0 - c) - z * s,
          x * z * (1.0 - c) + y * s,
        ],
        [
          x * y * (1.0 - c) + z * s,
          c + y * y * (1.0 - c),
          y * z * (1.0 - c) - x * s,
        ],
        [
          x * z * (1.0 - c) - y * s,
          y * z * (1.0 - c) + x * s,
          c + z * z * (1.0 - c),
        ],
      ],
    )
  }
}

impl<T: Sdf<3>> SdfRotate3D for T {}

#[derive(Clone, Copy, PartialEq)]
pub struct Rotate3D<T>(T, [[f64; 3]; 3]);

impl<T: Default> Default for Rotate3D<T> {
  fn default() -> Self {
    Self(
      T::default(),
      [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    )
  }
}

impl<T: Sdf<3>> Sdf<3> for Rotate3D<T> {
  #[inline]
  fn call(&self, pos: Vector<3>) -> f64 {
    self.0.call(self.1 * pos)
  }

  #[inline]
  fn hits(&self, pos: Vector<3>) -> bool {
    self.0.hits(self.1 * pos)
  }
}

impl<T: SdfInfo<3>> SdfInfo<3> for Rotate3D<T> {
  type Info = T::Info;
  #[inline]
  fn call_info(&self, pos: Vector<3>) -> (f64, Self::Info) {
    self.0.call_info(self.1 * pos)
  }
}

impl<T: SdfGrad<3>> SdfGrad<3> for Rotate3D<T> {
  #[inline]
  fn call_grad(&self, pos: Vector<3>) -> (f64, Vector<3>) {
    self.0.call_grad(self.1 * pos)
  }
}
