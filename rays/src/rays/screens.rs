use marchrs_iterators::MultiDims;
use marchrs_vectors::{GridIter, Vector};

#[derive(Clone, Copy, PartialEq)]
pub struct ScreenInfo<const N: usize> {
  pub dims: Vector<N>,
  pub res: MultiDims<N>,
}

impl<const N: usize> ScreenInfo<N> {
  pub fn new(dims: impl Into<Vector<N>>, res: impl Into<MultiDims<N>>) -> Self {
    Self {
      dims: dims.into(),
      res: res.into(),
    }
  }

  pub fn positions(&self) -> GridIter<N> {
    let last = self.dims * 0.5;
    GridIter::new(-last..=last, self.res)
  }
}
