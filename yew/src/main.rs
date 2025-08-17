use marchrs_rays::{PerspectiveCamera3D, ScreenInfo, Solid, SphereMarcher, BLACK};
use yew::prelude::*;

// mod configs;
mod screen;
use screen::*;
mod models;
use models::*;

#[function_component(App)]
pub fn app() -> Html {
  let marcher = SphereMarcher {
    max_iter: 50,
    ..Default::default()
  };
  let camera = PerspectiveCamera3D::default();
  let screen = ScreenInfo::new([0.3, 0.2], [1200, 800]);
  let renderer = Solid { background: BLACK };

  html! {
      <main>
          <h1>{ "Ray Marching!" }</h1>
          <Screen
            {marcher}
            source={camera}
            {screen}
            model={both()}
            {renderer}
          />
      </main>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
