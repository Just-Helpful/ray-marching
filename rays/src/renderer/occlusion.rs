use marchrs_sdf::traits::SdfFull;
use marchrs_vectors::Vector;

use crate::MarchError;

use super::Renderer;

pub struct Occlusion {
  pub num_iters: usize,
  pub step_size: f64,
}

impl<const N: usize> Renderer<N, f64> for Occlusion {
  fn render<S: SdfFull<N, Info = f64>>(
    &self,
    model: &S,
    hit: Result<Vector<N>, MarchError>,
  ) -> S::Info {
    let Ok(pos) = hit else { return 0.0 };
    let normal = model.grad(pos);
    (1..=self.num_iters)
      .map(|i| i as f64)
      .map(|i| {
        let step_dist = i * self.step_size;
        let dist = step_dist - model.call(pos + step_dist * normal);
        (dist * dist) / i
      })
      .sum()
  }
}
