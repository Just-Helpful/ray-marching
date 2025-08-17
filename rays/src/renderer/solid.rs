use super::Renderer;
use crate::MarchError;
use marchrs_sdf::traits::SdfFull;
use marchrs_vectors::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Solid<C> {
  pub background: C,
}

impl<const N: usize, C: Copy> Renderer<N, C> for Solid<C> {
  fn render<S: SdfFull<N, Info = C>>(
    &self,
    model: &S,
    hit: Result<Vector<N>, MarchError>,
  ) -> S::Info {
    let Ok(pos) = hit else { return self.background };
    model.info(pos)
  }
}
