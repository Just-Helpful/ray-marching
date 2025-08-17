extern crate marchrs_iterators;
extern crate marchrs_rays;
extern crate marchrs_sdf;
extern crate marchrs_vectors;

use std::{
  f64::consts::PI,
  fmt::{Display, Formatter},
};

use marchrs_iterators::MultiRangeIter;
use marchrs_rays::{
  CameraPlane, PerspectiveCamera3D, RayMarcher, RaySource, ScreenInfo, SphereMarcher,
};
use marchrs_sdf::{items::*, traits::*};
use marchrs_vectors::Vector;

#[derive(Clone, Copy)]
struct Colour([u8; 3]);

const RED: Colour = Colour([255, 0, 0]);
const BLACK: Colour = Colour([0, 0, 0]);

impl Colour {
  fn colour_text(&self, text: &str) -> String {
    let Colour([r, g, b]) = self;
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text)
  }
}

impl Display for Colour {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.colour_text("█"))
  }
}

//

fn get_model() -> DynModel<3, Colour> {
  Sphere.scale([0.1; 3]).with(RED).wrap()
}

fn get_camera() -> PerspectiveCamera3D {
  let plane = CameraPlane::new([-0.5, 0.0, 0.0], [1.0, 0.0, 0.0]);
  let upwards = Vector::from([0.0, 0.0, 1.0]);
  PerspectiveCamera3D::new(plane, upwards, PI / 8.0)
}

fn get_marcher() -> impl RayMarcher<3> {
  SphereMarcher {
    hit_error: 1e-4,
    miss_error: 1e2,
    max_iter: 20,
  }
}

fn render() -> Vec<Vec<Colour>> {
  let screen_info = ScreenInfo::new([0.3, 0.2], [150, 100]);
  let model = get_model();
  let camera = get_camera();
  let marcher = get_marcher();

  let rendered: Vec<_> = camera
    .rays(&screen_info)
    .map(|ray| marcher.march(&model, ray))
    .collect();

  let mut screen = vec![vec![BLACK; screen_info.res[0]]; screen_info.res[1]];
  for ([i, j], opt_colour) in MultiRangeIter::from(screen_info.res).zip(rendered) {
    let Ok(hit_pos) = opt_colour else { continue };
    screen[j][i] = model.info(hit_pos)
  }

  screen
}

pub fn main() {
  for _ in 0..500 {
    render();
  }
}
