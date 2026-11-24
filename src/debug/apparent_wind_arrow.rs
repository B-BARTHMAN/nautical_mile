use avian3d::prelude::LinearVelocity;
use bevy::prelude::{Gizmos, LinearRgba, Res, Single, Transform, Vec3, With};

use crate::sailing::boat::Boat;
use crate::wind::apparent::apparent_wind;
use crate::wind::state::Wind;

pub fn draw_apparent_wind(
  mut gizmos: Gizmos,
  wind: Res<Wind>,
  boat: Single<(&Transform, &LinearVelocity), With<Boat>>,
) {
  let (transform, velocity) = *boat;
  let start = transform.translation + Vec3::Y * 4.0;
  let end = start + apparent_wind(wind.velocity, velocity.0);
  gizmos.arrow(start, end, LinearRgba::rgb(0.0, 1.0, 1.0));
}
