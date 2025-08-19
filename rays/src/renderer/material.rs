use super::Rgba;

pub struct Material {
  /// The base colour of the item
  pub albedo: Rgba,
  /// The microsurface colour of the material,\
  /// I don't honestly know *how* to render this,\
  /// hopefully I'll work it out...
  pub surface: Rgba,
  /// The colour surface reflections are multiplied by
  pub reflect: Rgba,
}
