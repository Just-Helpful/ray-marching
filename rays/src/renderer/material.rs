use super::Rgba;

pub struct Material {
  /// The base colour of the item
  pub albedo: Rgba,
  /// The microsurface colour of the material,<br>
  /// I don't honestly know *how* to render this,<br>
  /// hopefully I'll work it out...
  pub surface: Rgba,
  /// The colour surface reflections are multiplied by
  pub reflect: Rgba,
}
