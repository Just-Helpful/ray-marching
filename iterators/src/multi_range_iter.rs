use crate::MultiDims;

use super::par_iter::{ParIter, Splittable};
use rayon::iter::IntoParallelIterator;

#[derive(Clone, Copy, Debug)]
pub struct MultiRangeIter<const N: usize> {
  pub dims: MultiDims<N>,
  pub range: Option<([usize; N], [usize; N])>,
}

impl<const N: usize, D: Into<MultiDims<N>>> From<D> for MultiRangeIter<N> {
  fn from(value: D) -> Self {
    let value: MultiDims<N> = value.into();
    assert!(
      value.iter().all(|&v| v > 0),
      "all dimensions should be non zero\ndimensions = {value:?}"
    );
    let mut end = value.0;
    for x in end.iter_mut() {
      *x -= 1;
    }
    MultiRangeIter {
      dims: value,
      range: Some(([0; N], end)),
    }
  }
}

impl<const N: usize> MultiRangeIter<N> {
  pub fn is_empty(&self) -> bool {
    self.range.is_none()
  }
}

// Sequential Iterators

impl<const N: usize> Iterator for MultiRangeIter<N> {
  type Item = [usize; N];

  fn next(&mut self) -> Option<Self::Item> {
    let (start, end) = self.range?;
    self.range = (start != end).then(|| (self.dims.increment(start), end));
    Some(start)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let Some((start, end)) = self.range else {
      return (0, Some(0));
    };
    let multi_len = self.dims.sub(end, start);
    let len = self.dims.into_flat(multi_len) + 1;
    (len, Some(len))
  }
}

impl<const N: usize> ExactSizeIterator for MultiRangeIter<N> {}

impl<const N: usize> DoubleEndedIterator for MultiRangeIter<N> {
  fn next_back(&mut self) -> Option<Self::Item> {
    let (start, end) = self.range?;
    self.range = (start != end).then(|| (start, self.dims.decrement(end)));
    Some(end)
  }
}

impl<const N: usize> Splittable for MultiRangeIter<N> {
  fn split_at(self, index: usize) -> (Self, Self) {
    let Some((start, end)) = self.range else {
      // iterator is empty, return empty iterator twice
      return (self, self);
    };
    let multi_index = self.dims.from_flat(index);
    let mid = self.dims.add(start, multi_index);
    (
      Self {
        range: Some((start, mid)),
        ..self
      },
      Self {
        range: Some((mid, end)),
        ..self
      },
    )
  }
}

impl<const N: usize> IntoParallelIterator for MultiRangeIter<N> {
  type Item = <Self as Iterator>::Item;
  type Iter = ParIter<Self>;
  fn into_par_iter(self) -> Self::Iter {
    ParIter(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::{array, prelude::*};

  fn multi_iter<const N: usize>(
    range: impl Strategy<Value = usize>,
  ) -> impl Strategy<Value = MultiRangeIter<N>> {
    array::uniform(range).prop_map_into()
  }

  fn idx_dims<const N: usize>(
    range: impl Strategy<Value = usize>,
  ) -> impl Strategy<Value = (usize, MultiRangeIter<N>)> {
    multi_iter(range).prop_flat_map(|arr| (0..arr.dims.iter().product(), Just(arr)))
  }

  fn split_iter_case<const N: usize>(
    idx: usize,
    iter: MultiRangeIter<N>,
  ) -> Result<(), TestCaseError> {
    let (iter0, iter1) = iter.split_at(idx);
    let len = iter.len();
    let len0 = iter0.len();
    prop_assert_eq!(len0, idx);
    let len1 = iter1.len();
    prop_assert_eq!(len0 + len1, len);
    Ok(())
  }

  proptest! {
    #[test]
    fn split_iter((idx, iter) in idx_dims::<5>(1usize..1000)) {
      split_iter_case(idx, iter)?;
    }

    #[test]
    fn par_collect(iter in multi_iter::<2>(1usize..5)) {
      let vec: Vec<_> = iter.collect();
      let par_iter = ParIter(iter);
      let par_vec: Vec<_> = par_iter.collect();
      prop_assert_eq!(vec, par_vec)
    }
  }

  #[test]
  fn split_iter_0() -> Result<(), TestCaseError> {
    split_iter_case(0, [1, 1, 1].into())
  }
}
