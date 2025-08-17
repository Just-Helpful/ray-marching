use super::{MarchError, Ray, RayMarcher};
use marchrs_sdf::traits::Sdf;
use marchrs_vectors::Vector;

#[derive(Clone, Copy, PartialEq)]
pub struct SphereMarcher {
  pub hit_error: f64,
  pub miss_error: f64,
  pub max_iter: usize,
}

impl Default for SphereMarcher {
  fn default() -> Self {
    Self {
      hit_error: 1e-4,
      miss_error: 1e4,
      max_iter: 20,
    }
  }
}

impl<const N: usize> RayMarcher<N> for SphereMarcher {
  fn march<S: Sdf<N>>(
    &self,
    sdf: &S,
    Ray { mut pos, dir }: Ray<N>,
  ) -> Result<Vector<N>, MarchError> {
    for _ in 0..self.max_iter {
      let dist = sdf.call(pos);
      if dist > self.miss_error {
        return Err(MarchError::Diverges(dist));
      }
      if dist < self.hit_error {
        return Ok(pos);
      }
      pos = pos + dist * dir;
    }

    Err(MarchError::MaxIter(self.max_iter))
  }
}
