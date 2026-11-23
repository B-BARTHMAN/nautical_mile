use bevy::camera::Camera3d;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Transform};

pub fn spawn_camera(mut commands: Commands) {
  commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
  ));
}
