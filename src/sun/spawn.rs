use bevy::light::DirectionalLight;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Transform};

pub fn spawn_sun(mut commands: Commands) {
  commands.spawn((
    DirectionalLight::default(),
    Transform::from_xyz(50.0, 100.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
  ));
}
