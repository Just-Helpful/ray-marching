use rayon::iter::{
  plumbing::{bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer},
  IndexedParallelIterator, ParallelIterator,
};
use std::ops::{Deref, DerefMut};

pub trait Splittable: Sized {
  fn split_at(self, index: usize) -> (Self, Self);
}

#[derive(Clone, Copy, Debug)]
pub struct ParIter<I>(pub I);

impl<I> Deref for ParIter<I> {
  type Target = I;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<I> DerefMut for ParIter<I> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

// Parallel Iterators

impl<I> ParallelIterator for ParIter<I>
where
  I: ExactSizeIterator + DoubleEndedIterator + Splittable + Send,
  I::Item: Send,
{
  type Item = I::Item;
  fn drive_unindexed<C>(self, consumer: C) -> C::Result
  where
    C: UnindexedConsumer<Self::Item>,
  {
    bridge(self, consumer)
  }

  fn opt_len(&self) -> Option<usize> {
    Some(self.len())
  }
}

impl<I> IndexedParallelIterator for ParIter<I>
where
  I: ExactSizeIterator + DoubleEndedIterator + Splittable + Send,
  I::Item: Send,
{
  fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
    callback.callback(self)
  }

  fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
    bridge(self, consumer)
  }

  fn len(&self) -> usize {
    ExactSizeIterator::len(&self.0)
  }
}

impl<I> Producer for ParIter<I>
where
  I: ExactSizeIterator + DoubleEndedIterator + Splittable + Send,
{
  type Item = I::Item;
  type IntoIter = I;

  fn into_iter(self) -> Self::IntoIter {
    self.0
  }

  fn split_at(self, index: usize) -> (Self, Self) {
    let (iter0, iter1) = self.0.split_at(index);
    (ParIter(iter0), ParIter(iter1))
  }
}
