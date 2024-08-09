use super::Vector;
use marchrs_iterators::{MultiDims, MultiRangeIter, ParIter, Splittable};
use rayon::iter::IntoParallelIterator;
use std::ops::RangeInclusive;

pub struct GridIter<const N: usize> {
  iter: MultiRangeIter<N>,
  start: Vector<N>,
  steps: Vector<N>,
}

impl<const N: usize> GridIter<N> {
  pub fn new(range: RangeInclusive<Vector<N>>, dims: impl Into<MultiDims<N>>) -> Self {
    let dims: MultiDims<N> = dims.into();
    let iter: MultiRangeIter<N> = dims.into();
    let start = *range.start();
    let end = *range.end();
    let steps = (end - start) / (Vector::from(dims.0) - 1.0);
    Self { iter, start, steps }
  }
}

impl<const N: usize> Iterator for GridIter<N> {
  type Item = Vector<N>;
  fn next(&mut self) -> Option<Self::Item> {
    let loc = self.iter.next()?;
    let pos: Vector<N> = loc.into();
    Some(self.start + self.steps * pos)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

impl<const N: usize> ExactSizeIterator for GridIter<N> {}

impl<const N: usize> DoubleEndedIterator for GridIter<N> {
  fn next_back(&mut self) -> Option<Self::Item> {
    let loc = self.iter.next_back()?;
    let pos: Vector<N> = loc.into();
    Some(self.start + self.steps * pos)
  }
}

impl<const N: usize> Splittable for GridIter<N> {
  fn split_at(self, index: usize) -> (Self, Self) {
    let (iter0, iter1) = self.iter.split_at(index);
    (
      Self {
        iter: iter0,
        ..self
      },
      Self {
        iter: iter1,
        ..self
      },
    )
  }
}

impl<const N: usize> IntoParallelIterator for GridIter<N> {
  type Item = <Self as Iterator>::Item;
  type Iter = ParIter<Self>;
  fn into_par_iter(self) -> Self::Iter {
    ParIter(self)
  }
}
