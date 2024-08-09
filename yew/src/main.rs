use marchrs_rays::{CameraPlane, PerspectiveCamera3D, ScreenInfo, SphereMarcher};
use marchrs_sdf::{items::*, traits::*};
use std::f64::consts::PI;
use yew::prelude::*;

// mod configs;
mod screen;
use screen::*;

#[function_component(App)]
pub fn app() -> Html {
  let marcher = SphereMarcher {
    max_iter: 50,
    ..Default::default()
  };
  let camera = PerspectiveCamera3D::new(
    CameraPlane::new([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
    [0.0, 0.0, 1.0].into(),
    PI / 8.0,
  );
  let screen = ScreenInfo::new([0.3, 0.2], [1200, 800]);

  let model = Cuboid::default()
    .scale([0.1; 3])
    .rot([1.0, 0.0, 0.0], PI / 3.0)
    .rot([0.0, 0.0, 1.0], PI / 4.0)
    .round(0.02)
    .with(FG)
    .wrap();

  html! {
      <main>
          <h1>{ "Ray Marching!" }</h1>
          <Screen {marcher} source={camera} {screen} {model} />
      </main>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
