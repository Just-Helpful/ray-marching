use marchrs_vectors::Vector;

pub trait Interpolate {
  fn lerp(self, rhs: Self, f: f64) -> Self;
}

impl Interpolate for f64 {
  #[inline]
  fn lerp(self, rhs: Self, f: f64) -> Self {
    (1.0 - f) * self + f * rhs
  }
}

impl Interpolate for bool {
  #[inline]
  fn lerp(self, rhs: Self, f: f64) -> Self {
    if f <= 0.5 {
      self
    } else {
      rhs
    }
  }
}

impl Interpolate for usize {
  #[inline]
  fn lerp(self, rhs: Self, f: f64) -> Self {
    let step = (rhs as f64 - self as f64) * f;
    if step >= 0.0 {
      self + (step.round() as usize)
    } else {
      let rounded = -step.round() as usize;
      assert!(rounded <= self, "Cannot extrapolate below 0 for `usize`s");
      self - rounded
    }
  }
}

impl<const N: usize, T: Interpolate> Interpolate for [T; N] {
  #[inline]
  fn lerp(mut self, rhs: Self, f: f64) -> Self {
    for (l, r) in self.iter_mut().zip(rhs) {
      take_mut::take(l, |l| l.lerp(r, f));
    }
    self
  }
}

impl<const N: usize> Interpolate for Vector<N> {
  #[inline]
  fn lerp(self, rhs: Self, f: f64) -> Self {
    Self(self.0.lerp(rhs.0, f))
  }
}
