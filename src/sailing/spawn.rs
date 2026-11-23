use crate::sailing::boat::Boat;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::mesh::{Mesh, Mesh3d};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, ResMut, Transform};
use bevy::shape::Cuboid;

pub fn spawn_boat(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut material: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Cuboid::new(3.0, 1.5, 9.0));
  let material = material.add(Color::srgb(0.9, 0.85, 0.75));

  commands.spawn((
    Boat::default(),
    Mesh3d(mesh),
    MeshMaterial3d(material),
    Transform::from_xyz(0.0, 0.25, 0.0),
  ));
}
