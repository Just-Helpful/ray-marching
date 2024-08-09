use marchrs_rays::{PerspectiveCamera3D, RayMarcher, RaySource, ScreenInfo, SphereMarcher};
use marchrs_sdf::traits::DynModel;
use rayon::iter::IntoParallelIterator;
use yew::prelude::*;

mod canvas_draw;
use canvas_draw::use_canvas_draw;

#[derive(Clone, Copy, PartialEq)]
pub struct Rgba([u8; 4]);
pub const BLACK: Rgba = Rgba([0, 0, 0, 255]);
pub const FG: Rgba = Rgba([0, 154, 91, 255]);

#[derive(Properties, PartialEq)]
pub struct ScreenProps {
  pub marcher: SphereMarcher,
  pub source: PerspectiveCamera3D,
  pub screen: ScreenInfo<2>,
  pub model: DynModel<3, Rgba>,
}

#[function_component]
pub fn Screen(props: &ScreenProps) -> Html {
  let ScreenProps {
    marcher,
    source,
    screen,
    model,
  } = props;

  let canvas_ref = use_node_ref();

  // run the rendering pipeline
  let render = use_memo(
    (*marcher, *source, *screen, model.clone()),
    |(marcher, source, screen, model)| -> Vec<_> {
      source
        .rays(screen)
        .into_par_iter()
        .map(|ray| marcher.call(model, ray))
        .map(|res| res.unwrap_or(BLACK))
        .collect()
    },
  );

  use_canvas_draw(canvas_ref.clone(), render, screen.res.0);

  html! {
    <canvas
      ref={canvas_ref}
      width={screen.res[0].to_string()}
      height={screen.res[1].to_string()}
    />
  }
}
