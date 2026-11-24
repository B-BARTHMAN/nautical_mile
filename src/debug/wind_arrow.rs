use bevy::prelude::{Gizmos, LinearRgba, Res, Single, Transform, Vec3, With};

use crate::sailing::boat::Boat;
use crate::wind::state::Wind;

pub fn draw_wind(mut gizmos: Gizmos, wind: Res<Wind>, boat: Single<&Transform, With<Boat>>) {
  let start = boat.translation + Vec3::Y * 4.0;
  gizmos.arrow(start, start + wind.velocity, LinearRgba::rgb(1.0, 1.0, 0.0));
}
