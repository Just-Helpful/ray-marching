use super::Vector;
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<const N: usize> Vector<N> {
  /// Helper method to map over a vector<br>
  /// used to make unary operators and binary operators with a float simpler.
  fn map(self, f: impl Fn(f64) -> f64) -> Self {
    Self(self.0.map(f))
  }

  /// Helper method to zip 2 vectors with a given function<br>
  /// used to make binary operators vastly simpler to implement.
  fn zip_with(mut self, rhs: Self, f: impl Fn(f64, f64) -> f64) -> Self {
    for (lhs, rhs) in self.iter_mut().zip(rhs) {
      *lhs = f(*lhs, rhs)
    }
    self
  }
}

impl<const N: usize> Neg for Vector<N> {
  type Output = Vector<N>;
  fn neg(self) -> Self::Output {
    self.map(Neg::neg)
  }
}

impl<const N: usize> Add<f64> for Vector<N> {
  type Output = Vector<N>;
  fn add(self, rhs: f64) -> Self::Output {
    self.map(|x| x + rhs)
  }
}

impl<const N: usize> Sub<f64> for Vector<N> {
  type Output = Vector<N>;
  fn sub(self, rhs: f64) -> Self::Output {
    self.map(|x| x - rhs)
  }
}

impl<const N: usize> Mul<f64> for Vector<N> {
  type Output = Vector<N>;
  fn mul(self, rhs: f64) -> Self::Output {
    self.map(|x| x * rhs)
  }
}

impl<const N: usize> Div<f64> for Vector<N> {
  type Output = Vector<N>;
  fn div(self, rhs: f64) -> Self::Output {
    self.map(|x| x / rhs)
  }
}

impl<const N: usize> Add<Vector<N>> for f64 {
  type Output = Vector<N>;
  fn add(self, rhs: Vector<N>) -> Self::Output {
    rhs.map(|x| self + x)
  }
}

impl<const N: usize> Sub<Vector<N>> for f64 {
  type Output = Vector<N>;
  fn sub(self, rhs: Vector<N>) -> Self::Output {
    rhs.map(|x| self - x)
  }
}

impl<const N: usize> Mul<Vector<N>> for f64 {
  type Output = Vector<N>;
  fn mul(self, rhs: Vector<N>) -> Self::Output {
    rhs.map(|x| self * x)
  }
}

impl<const N: usize> Div<Vector<N>> for f64 {
  type Output = Vector<N>;
  fn div(self, rhs: Vector<N>) -> Self::Output {
    rhs.map(|x| self / x)
  }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> {
  type Output = Vector<N>;
  fn add(self, rhs: Vector<N>) -> Self::Output {
    self.zip_with(rhs, Add::add)
  }
}

impl<const N: usize> Sub<Vector<N>> for Vector<N> {
  type Output = Vector<N>;
  fn sub(self, rhs: Vector<N>) -> Self::Output {
    self.zip_with(rhs, Sub::sub)
  }
}

impl<const N: usize> Mul<Vector<N>> for Vector<N> {
  type Output = Vector<N>;
  fn mul(self, rhs: Vector<N>) -> Self::Output {
    self.zip_with(rhs, Mul::mul)
  }
}

impl<const N: usize> Div<Vector<N>> for Vector<N> {
  type Output = Vector<N>;
  fn div(self, rhs: Vector<N>) -> Self::Output {
    self.zip_with(rhs, Div::div)
  }
}

impl Vector<3> {
  pub fn cross(self, rhs: Self) -> Self {
    let Vector([a0, a1, a2]) = self;
    let Vector([b0, b1, b2]) = rhs;
    Vector([a1 * b2 - a2 * b1, a2 * b0 - a0 * b2, a0 * b1 - a1 * b0])
  }
}
