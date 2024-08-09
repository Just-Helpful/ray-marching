use super::Vector;

impl<const N: usize> From<[f64; N]> for Vector<N> {
  fn from(value: [f64; N]) -> Self {
    Self(value)
  }
}

impl<const N: usize> From<f64> for Vector<N> {
  fn from(value: f64) -> Self {
    Self([value; N])
  }
}

impl<const N: usize> From<[usize; N]> for Vector<N> {
  fn from(value: [usize; N]) -> Self {
    Self(value.map(|x| x as f64))
  }
}

impl<const N: usize> Vector<N> {
  #[inline]
  pub fn zeros() -> Self {
    Self([0.0; N])
  }

  #[inline]
  pub fn ones() -> Self {
    Self([1.0; N])
  }

  #[inline]
  pub fn axis(i: usize) -> Self {
    debug_assert!(
      i < N,
      "Can only construct axis vectors for axes [0, {N})\n\
      Got axis {i} >= {N}.",
    );
    let mut arr = [0.0; N];
    arr[i] = 1.0;
    Self(arr)
  }
}
