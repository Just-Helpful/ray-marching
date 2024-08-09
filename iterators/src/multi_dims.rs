use std::{
  array,
  ops::{Deref, DerefMut},
};

/// A multi dimensional version of `RangeTo`.
/// This is effectively a `[0..size; N]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MultiDims<const N: usize>(pub [usize; N]);

impl<const N: usize> From<[usize; N]> for MultiDims<N> {
  fn from(value: [usize; N]) -> Self {
    MultiDims(value)
  }
}

impl<const N: usize> Deref for MultiDims<N> {
  type Target = [usize; N];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<const N: usize> DerefMut for MultiDims<N> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<const N: usize> MultiDims<N> {
  pub fn into_flat(&self, idx: [usize; N]) -> usize {
    let mut flat = idx[N - 1];
    for i in (0..N - 1).rev() {
      flat *= self[i];
      flat += idx[i]
    }
    flat
  }

  pub fn from_flat(&self, mut idx: usize) -> [usize; N] {
    self.map(|dim| {
      let res = idx % dim;
      idx /= dim;
      res
    })
  }

  pub fn len(&self) -> usize {
    self.iter().product()
  }

  pub fn is_empty(&self) -> bool {
    self.iter().any(|&dim| dim == 0)
  }

  /// The minimum possible value within this range
  pub fn min(&self) -> [usize; N] {
    [0; N]
  }

  /// The maximum possible value within this range
  pub fn max(&self) -> [usize; N] {
    self.map(|dim| dim - 1)
  }

  /// Increments the provided index
  pub fn increment(&self, mut idx: [usize; N]) -> [usize; N] {
    for (x, dim) in idx.iter_mut().zip(self.into_iter()) {
      *x += 1;
      if *x < dim {
        break;
      }
      *x = 0
    }
    idx
  }

  /// Decrements the provided index
  pub fn decrement(&self, mut idx: [usize; N]) -> [usize; N] {
    for (x, dim) in idx.iter_mut().zip(self.into_iter()) {
      if *x > 0 {
        *x -= 1;
        break;
      }
      *x = dim - 1;
    }
    idx
  }

  /// Adds 2 multi-indexes, least significant digit first.
  ///
  /// This **doesn't** convert to flat indexes, to avoid integer overflows.
  pub fn add(&self, lhs: [usize; N], rhs: [usize; N]) -> [usize; N] {
    let mut carry = 0;
    array::from_fn(|i| {
      let res = lhs[i] + rhs[i] + carry;
      carry = res / self[i];
      res % self[i]
    })
  }

  /// Subtracts 2 multi-indexes, least significant digit first.
  ///
  /// This **doesn't** convert to flat indexes, to avoid integer overflows.
  pub fn sub(&self, lhs: [usize; N], rhs: [usize; N]) -> [usize; N] {
    let mut carry = 0;
    array::from_fn(|i| {
      let (l, dim) = (lhs[i], self[i]);
      let r = rhs[i] + carry;
      if l >= r {
        return l - r;
      }

      let diff = r - l;
      carry = 1 + diff / dim;
      dim - diff % dim
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::{array, prelude::*};

  fn multi_dims<const N: usize>(
    range: impl Strategy<Value = usize>,
  ) -> impl Strategy<Value = MultiDims<N>> {
    array::uniform(range).prop_map_into()
  }

  fn with_flat<const N: usize>(
    dims: impl Strategy<Value = MultiDims<N>>,
  ) -> impl Strategy<Value = (MultiDims<N>, usize)> {
    dims.prop_flat_map(|dims| (Just(dims), 0..dims.len()))
  }

  fn with_flat2<const N: usize>(
    dims: impl Strategy<Value = MultiDims<N>>,
  ) -> impl Strategy<Value = (MultiDims<N>, (usize, usize))> {
    dims.prop_flat_map(|dims| {
      (0..dims.len()).prop_flat_map(move |idx| (Just(dims), (Just(idx), 0..(dims.len() - idx))))
    })
  }

  fn with_multi<const N: usize>(
    dims: impl Strategy<Value = MultiDims<N>>,
  ) -> impl Strategy<Value = (MultiDims<N>, [usize; N])> {
    dims.prop_flat_map(|dims| {
      (
        Just(dims),
        (0..dims.len()).prop_map(move |flat| dims.from_flat(flat)),
      )
    })
  }

  proptest! {
    #[test]
    fn flat_idx((dims, flat) in with_flat::<5>(multi_dims(1usize..1000))) {
      prop_assert_eq!(dims.into_flat(dims.from_flat(flat)), flat);
    }

    #[test]
    fn increment_flat((dims, flat) in with_flat::<5>(multi_dims(1usize..1000))) {
      prop_assume!(flat < dims.len() - 1);
      let multi = dims.from_flat(flat);
      let multi = dims.increment(multi);
      let flat_incr = dims.into_flat(multi);
      prop_assert_eq!(flat_incr, flat + 1)
    }

    #[test]
    fn increment_add_one((dims, flat) in with_flat::<5>(multi_dims(1usize..1000))) {
      prop_assume!(flat < dims.len() - 1);
      let one = dims.from_flat(1);
      let multi = dims.from_flat(flat);
      prop_assert_eq!(dims.increment(multi), dims.add(multi, one))
    }

    #[test]
    fn decrement_flat((dims, flat) in with_flat::<5>(multi_dims(1usize..1000))) {
      prop_assume!(flat > 0);
      let multi = dims.from_flat(flat);
      let multi = dims.decrement(multi);
      let flat_decr = dims.into_flat(multi);
      prop_assert_eq!(flat_decr, flat - 1)
    }

    #[test]
    fn add_identity((dims, multi) in with_multi::<5>(multi_dims(1usize..1000))) {
      prop_assert_eq!(dims.add([0; 5], multi), multi)
    }

    #[test]
    fn add_sub_identity((dims, (flat0, flat1)) in with_flat2::<2>(multi_dims(1usize..1000))) {
      let multi0 = dims.from_flat(flat0);
      let multi1 = dims.from_flat(flat1);
      let multi_add = dims.add(multi0, multi1);
      let multi_sub = dims.sub(multi_add, multi1);
      prop_assert_eq!(multi_sub, multi0);
    }
  }
}
