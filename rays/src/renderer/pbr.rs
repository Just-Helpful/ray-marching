use super::{Occlusion, Rgba, Solid};

pub struct PBR {
  pub solid: Solid<Rgba>,
  pub occlusion: Occlusion,
}
