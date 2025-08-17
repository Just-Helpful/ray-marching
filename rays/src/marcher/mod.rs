use std::fmt::{Display, Formatter};

use crate::sources::Ray;
use marchrs_sdf::traits::Sdf;

mod sphere_march;
use marchrs_vectors::Vector;
pub use sphere_march::*;

/// An error indicating the ray marcher didn't hit an object
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MarchError {
  /// Ray marching diverged, reaching the given distance from any surface
  Diverges(f64),
  /// Ray marching didn't hit a surface in the given number of iterations
  MaxIter(usize),
}

impl Display for MarchError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    use MarchError::*;
    match self {
      Diverges(dist) => write!(f, "Diverged by stepping {dist} distance"),
      MaxIter(iters) => write!(f, "Failed to converge after {iters} iterations"),
    }
  }
}

pub trait RayMarcher<const N: usize> {
  fn march<S: Sdf<N>>(&self, sdf: &S, ray: Ray<N>) -> Result<Vector<N>, MarchError>;
}
