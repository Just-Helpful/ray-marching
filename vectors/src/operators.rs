use super::Vector;
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<const N: usize> Neg for Vector<N> {
  type Output = Vector<N>;
  fn neg(self) -> Self::Output {
    Self(self.0.map(|x| -x))
  }
}

impl<const N: usize> Add<f64> for Vector<N> {
  type Output = Vector<N>;
  fn add(self, rhs: f64) -> Self::Output {
    Self(self.0.map(|l| l + rhs))
  }
}

impl<const N: usize> Sub<f64> for Vector<N> {
  type Output = Vector<N>;
  fn sub(self, rhs: f64) -> Self::Output {
    Self(self.0.map(|l| l - rhs))
  }
}

impl<const N: usize> Mul<f64> for Vector<N> {
  type Output = Vector<N>;
  fn mul(self, rhs: f64) -> Self::Output {
    Self(self.0.map(|l| l * rhs))
  }
}

impl<const N: usize> Div<f64> for Vector<N> {
  type Output = Vector<N>;
  fn div(self, rhs: f64) -> Self::Output {
    Self(self.0.map(|l| l / rhs))
  }
}

impl<const N: usize> Add<Vector<N>> for f64 {
  type Output = Vector<N>;
  fn add(self, rhs: Vector<N>) -> Self::Output {
    Vector(rhs.0.map(|r| self + r))
  }
}

impl<const N: usize> Sub<Vector<N>> for f64 {
  type Output = Vector<N>;
  fn sub(self, rhs: Vector<N>) -> Self::Output {
    Vector(rhs.0.map(|r| self - r))
  }
}

impl<const N: usize> Mul<Vector<N>> for f64 {
  type Output = Vector<N>;
  fn mul(self, rhs: Vector<N>) -> Self::Output {
    Vector(rhs.0.map(|r| self * r))
  }
}

impl<const N: usize> Div<Vector<N>> for f64 {
  type Output = Vector<N>;
  fn div(self, rhs: Vector<N>) -> Self::Output {
    Vector(rhs.0.map(|r| self / r))
  }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> {
  type Output = Vector<N>;
  fn add(mut self, rhs: Vector<N>) -> Self::Output {
    for (mut_l, r) in self.iter_mut().zip(rhs) {
      *mut_l += r;
    }
    self
  }
}

impl<const N: usize> Sub<Vector<N>> for Vector<N> {
  type Output = Vector<N>;
  fn sub(mut self, rhs: Vector<N>) -> Self::Output {
    for (mut_l, r) in self.iter_mut().zip(rhs) {
      *mut_l -= r;
    }
    self
  }
}

impl<const N: usize> Mul<Vector<N>> for Vector<N> {
  type Output = Vector<N>;
  fn mul(mut self, rhs: Vector<N>) -> Self::Output {
    for (mut_l, r) in self.iter_mut().zip(rhs) {
      *mut_l *= r;
    }
    self
  }
}

impl<const N: usize> Div<Vector<N>> for Vector<N> {
  type Output = Vector<N>;
  fn div(mut self, rhs: Vector<N>) -> Self::Output {
    for (mut_l, r) in self.iter_mut().zip(rhs) {
      *mut_l /= r;
    }
    self
  }
}

impl Vector<3> {
  pub fn cross(self, rhs: Self) -> Self {
    let Vector([a0, a1, a2]) = self;
    let Vector([b0, b1, b2]) = rhs;
    Vector([a1 * b2 - a2 * b1, a2 * b0 - a0 * b2, a0 * b1 - a1 * b0])
  }
}
