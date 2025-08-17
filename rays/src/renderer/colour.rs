#[derive(Clone, Copy, PartialEq)]
pub struct Rgba(pub [f32; 4]);

impl From<[u8; 4]> for Rgba {
  fn from(value: [u8; 4]) -> Self {
    Rgba(value.map(|x| (x as f32 + 0.5) / 256.0))
  }
}

impl From<Rgba> for [u8; 4] {
  fn from(value: Rgba) -> Self {
    value.0.map(|x| (x * 256.0).floor() as u8)
  }
}

pub const BLACK: Rgba = Rgba([0.0, 0.0, 0.0, 1.0]);
pub const WHITE: Rgba = Rgba([1.0; 4]);
