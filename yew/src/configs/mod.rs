use std::ops::{Deref, RangeInclusive};

use marchrs_vectors::Vector;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{functional::Hook, hook, html, use_effect, use_state, Callback, Event, Html};

mod rays;

pub trait HtmlConfig: Sized {
  fn use_config<'hook>(self) -> impl 'hook + Hook<Output = (Self, Html)>;
}

#[hook]
fn use_f64_config(init: f64, range: RangeInclusive<f64>) -> (f64, Html) {
  let value = use_state(|| init);
  let min = format!("{}", range.start());
  let max = format!("{}", range.end());
  let onchange = {
    let value = value.clone();
    Callback::from(move |event: Event| {
      let target = event.target().expect("onchange should have target");
      let elem: HtmlInputElement = target.unchecked_into();
      let val = elem.value_of().as_f64().expect("range input should be f64");
      value.set(val)
    })
  };
  (
    *value,
    html! {
      <input type="range" {onchange} {min} {max} value={value.to_string()} />
    },
  )
}

#[hook]
fn use_vec_config<const N: usize>(
  init: Vector<N>,
  ranges: [RangeInclusive<f64>; N],
) -> (Vector<N>, Html) {
  let vector = use_state(|| init);
  let inputs = vec![];

  for (init, range) in init.into_iter().zip(ranges) {
    use_effect()
  }

  (*vector, html! {<>{inputs}</>})
}
