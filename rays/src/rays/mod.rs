use marchrs_vectors::Vector;
use rayon::iter::IntoParallelIterator;

mod cameras;
pub use cameras::PerspectiveCamera3D;
mod screens;
pub use screens::ScreenInfo;

/// A simple definition of a plane
#[derive(Clone, Copy, PartialEq)]
pub struct CameraPlane<const N: usize> {
  pub pos: Vector<N>,
  pub normal: Vector<N>,
}

impl<const N: usize> CameraPlane<N> {
  pub fn new(pos: impl Into<Vector<N>>, normal: impl Into<Vector<N>>) -> Self {
    Self {
      pos: pos.into(),
      normal: normal.into().normal(),
    }
  }
}

/// A Ray to be cast into a scene
#[derive(Clone, Copy)]
pub struct Ray<const N: usize> {
  pub pos: Vector<N>,
  pub dir: Vector<N>,
}

/// Any object that can emit rays in a scene
pub trait RaySource<const N: usize> {
  type Screen;
  type RayIter: Iterator<Item = Ray<N>> + IntoParallelIterator<Item = Ray<N>>;
  fn rays(&self, screen: &Self::Screen) -> Self::RayIter;
}
