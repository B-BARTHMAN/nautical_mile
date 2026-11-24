use std::f32::consts::FRAC_PI_2;

use bevy::prelude::{Gizmos, Gray, Isometry3d, LinearRgba, Quat, UVec2, Vec2, Vec3};

pub fn draw_grid(mut gizmos: Gizmos) {
  gizmos.grid(
    Isometry3d::new(Vec3::new(0.0, 0.05, 0.0), Quat::from_rotation_x(FRAC_PI_2)),
    UVec2::splat(100),
    Vec2::splat(10.0),
    LinearRgba::gray(0.65),
  );
}
