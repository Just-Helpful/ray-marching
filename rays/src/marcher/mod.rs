use std::fmt::{Display, Formatter};

use crate::rays::Ray;
use marchrs_sdf::traits::SdfInfo;

mod sphere_march;
pub use sphere_march::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MarchError {
  Diverges(f64),
  NoHit(usize),
}

impl Display for MarchError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    use MarchError::*;
    match self {
      Diverges(dist) => write!(f, "Diverged by stepping {dist} distance"),
      NoHit(iters) => write!(f, "Failed to converge after {iters} iterations"),
    }
  }
}

pub trait RayMarcher<const N: usize> {
  fn call<S: SdfInfo<N>>(&self, sdf: &S, ray: Ray<N>) -> Result<S::Info, MarchError>;
}
