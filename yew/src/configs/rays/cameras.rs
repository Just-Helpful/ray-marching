use crate::configs::HtmlConfig;
use marchrs_rays::PerspectiveCamera3D;
use yew::{functional::Hook, hook, html, Html};

#[hook]
fn use_perspective_config(
  PerspectiveCamera3D {
    plane,
    upwards,
    screen,
    fov,
  }: PerspectiveCamera3D,
) -> (PerspectiveCamera3D, Html) {
  let (plane, plane_html) = plane.use_config();
  let (upwards, upwards_html) = upwards.use_config();
  let (screen, screen_html) = screen.use_config();
  let (fov, fov_html) = fov.use_config();

  (
    PerspectiveCamera3D {
      plane,
      upwards,
      screen,
      fov,
    },
    html! {
      <>
        {plane_html}
        {upwards_html}
        {screen_html}
        {fov_html}
      </>
    },
  )
}

impl HtmlConfig for PerspectiveCamera3D {
  fn use_config<'hook>(self) -> impl 'hook + Hook<Output = (Self, Html)> {
    use_perspective_config(self)
  }
}
