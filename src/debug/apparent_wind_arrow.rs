use crate::physics::wind::apparent_wind;
use crate::sailing::boat::Boat;
use crate::wind::state::Wind;
use bevy::prelude::{Gizmos, LinearRgba, Res, Single, Transform, Vec3};

pub fn draw_apparent_wind(mut gizmos: Gizmos, wind: Res<Wind>, boat: Single<(&Transform, &Boat)>) {
  let (transform, boat) = *boat;
  let start = transform.translation + Vec3::Y * 4.0;
  let end = start + apparent_wind(wind.velocity, boat.velocity);
  gizmos.arrow(start, end, LinearRgba::rgb(0.0, 1.0, 1.0));
}
