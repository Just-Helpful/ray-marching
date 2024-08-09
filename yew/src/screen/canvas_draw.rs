use super::Rgba;
use std::rc::Rc;
use wasm_bindgen::{Clamped, JsCast, UnwrapThrowExt};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use yew::prelude::*;

/// Writes pixels to the provided canvas, when they update.
///
/// - `node`: the canvas element to draw pixels on
/// - `pixels`: the pixel buffer to draw to the canvas
/// - `dims`: the width & height of the image data to draw
///
/// This hook will redraw on changes to `pixels` or `dims`.
#[hook]
pub fn use_canvas_draw(node: NodeRef, pixels: Rc<Vec<Rgba>>, dims: [usize; 2]) {
  let pixel_data = use_memo(pixels, |pixels| -> Vec<_> {
    pixels.iter().flat_map(|rgba| rgba.0.into_iter()).collect()
  });

  use_effect_with(
    (pixel_data, dims[0] as u32, dims[1] as u32),
    move |(pixels, w, h)| {
      let canvas = node
        .cast::<HtmlCanvasElement>()
        .expect_throw("NodeRef should be to a Canvas element");
      let ctx = canvas
        .get_context("2d")
        .expect_throw("Canvas should have a 2d context")
        .expect_throw("Canvas 2d context should be non-null")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect_throw("`get_context(\"2d\")` should give a 2d context");

      let pixels = Clamped(pixels.as_slice());
      let data = ImageData::new_with_u8_clamped_array_and_sh(pixels, *w, *h)
        .expect_throw("Should be able to construct ImageData");

      ctx
        .put_image_data(&data, 0.0, 0.0)
        .expect_throw("Should be able to draw image in canvas context")
    },
  )
}
