use std::f64::consts::PI;

use super::{screens::ScreenInfo, CameraPlane, Ray, RaySource};
use marchrs_vectors::{GridIter, Vector};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Copy, PartialEq)]
pub struct PerspectiveCamera3D {
  pub plane: CameraPlane<3>,
  pub upwards: Vector<3>,
  pub fov: f64,
}

impl Default for PerspectiveCamera3D {
  fn default() -> Self {
    Self {
      plane: CameraPlane::new([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
      upwards: Vector([0.0, 0.0, 1.0]),
      fov: PI / 8.0,
    }
  }
}

impl PerspectiveCamera3D {
  pub fn new(plane: CameraPlane<3>, upwards: Vector<3>, fov: f64) -> Self {
    PerspectiveCamera3D {
      plane,
      upwards: upwards.normal(),
      fov,
    }
  }

  /// Calculates the distance the camera position<br>.
  /// We choose this such that the FOV angle is over the largest dimension.
  fn camera_pos(&self, screen: &ScreenInfo<2>) -> Vector<3> {
    let max_dim = screen.dims.max();
    let cam_dist = max_dim / (2.0 * (self.fov / 2.0).tan());
    self.plane.pos - cam_dist * self.plane.normal
  }

  /// Calculates a rightwards vector based on the camera normal and up vector.
  fn right(&self) -> Vector<3> {
    self.plane.normal.cross(self.upwards)
  }
}

impl RaySource<3> for PerspectiveCamera3D {
  type Screen = ScreenInfo<2>;
  type RayIter = Perspective3DIter;
  fn rays(&self, screen: &Self::Screen) -> Self::RayIter {
    let iter = screen.positions();
    Perspective3DIter {
      camera_pos: self.camera_pos(screen),
      screen_pos: self.plane.pos,
      basis: [self.right(), self.upwards],
      iter,
    }
  }
}

pub struct Perspective3DIter {
  camera_pos: Vector<3>,
  screen_pos: Vector<3>,
  basis: [Vector<3>; 2],
  iter: GridIter<2>,
}

impl Iterator for Perspective3DIter {
  type Item = Ray<3>;
  fn next(&mut self) -> Option<Self::Item> {
    let Vector([i, j]) = self.iter.next()?;
    let [u, v] = self.basis;

    // position in world space
    let pos = i * u + j * v + self.screen_pos;
    Some(Ray {
      pos,
      dir: pos - self.camera_pos,
    })
  }
}

impl ParallelIterator for Perspective3DIter {
  type Item = Ray<3>;
  fn drive_unindexed<C>(self, consumer: C) -> C::Result
  where
    C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
  {
    self
      .iter
      .into_par_iter()
      .map(|Vector([i, j])| {
        let [u, v] = self.basis;
        let pos = i * u + j * v + self.screen_pos;
        Ray {
          pos,
          dir: pos - self.camera_pos,
        }
      })
      .drive_unindexed(consumer)
  }
}
