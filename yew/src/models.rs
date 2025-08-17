use marchrs_rays::Rgba;
use marchrs_sdf::{items::*, traits::*};
use std::f64::consts::PI;

pub const FG: [u8; 4] = [0, 154, 91, 255];

pub fn smoothed_cube() -> DynModel<3, Rgba> {
  Cuboid::default()
    .scale([0.1; 3])
    .rot([1.0, 0.0, 0.0], PI / 3.0)
    .rot([0.0, 0.0, 1.0], PI / 4.0)
    .round(0.02)
    .with(Rgba::from(FG))
    .wrap()
}

pub fn joined_spheres() -> DynModel<3, Rgba> {
  let sphere = Sphere.scale([0.075; 3]);
  SmoothUnion(
    sphere.translate([0.0, 0.0, -0.03]),
    sphere.translate([0.0, 0.0, 0.03]),
    2.5e-3,
  )
  .rot([1.0, 0.0, 0.0], PI / 4.0)
  .with(Rgba::from(FG))
  .wrap()
}

pub fn both() -> DynModel<3, Rgba> {
  let cube = smoothed_cube();
  let spheres = joined_spheres();
  Union(
    cube.translate([0.0, 0.1, 0.0]),
    spheres.translate([0.0, -0.1, 0.0]),
  )
  .wrap()
}
