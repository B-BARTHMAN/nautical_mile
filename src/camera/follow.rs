use crate::sailing::boat::Boat;
use bevy::camera::Camera3d;
use bevy::math::Vec3;
use bevy::prelude::{Single, Transform, With, Without};

const OFFSET: Vec3 = Vec3::new(0.0, 4.0, 14.0);

pub fn follow_boat(
  boat: Single<&Transform, (With<Boat>, Without<Camera3d>)>,
  mut camera: Single<&mut Transform, (With<Camera3d>, Without<Boat>)>,
) {
  camera.translation = boat.transform_point(OFFSET);
  camera.look_at(boat.translation, Vec3::Y);
}
