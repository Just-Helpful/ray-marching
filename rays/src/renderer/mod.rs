use crate::MarchError;
use marchrs_sdf::traits::SdfFull;
use marchrs_vectors::Vector;

mod colour;
pub use colour::*;
mod material;
pub use material::*;
mod occlusion;
pub use occlusion::*;
mod pbr;
pub use pbr::*;
mod solid;
pub use solid::*;

/// A method of rendering the information for a single ray hit
pub trait Renderer<const N: usize, I> {
  /// Renders a given `SDF` item within a scene,\
  /// Returning the render information for the closest item.
  fn render<S: SdfFull<N, Info = I>>(
    &self,
    model: &S,
    hit: Result<Vector<N>, MarchError>,
  ) -> S::Info;
}
