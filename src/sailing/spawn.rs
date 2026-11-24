use avian3d::prelude::{Collider, Mass, RigidBody};
use bevy::prelude::{
  Assets, Color, Commands, Cuboid, Mesh, Mesh3d, MeshMaterial3d, ResMut, StandardMaterial,
  Transform,
};

use crate::sailing::boat::Boat;
use crate::sailing::buoyancy::box_probes;

pub fn spawn_boat(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Cuboid::new(3.0, 1.5, 9.0));
  let material = materials.add(Color::srgb(0.9, 0.85, 0.75));

  commands.spawn((
    Boat,
    RigidBody::Dynamic,
    Mass(4000.0),
    Collider::cuboid(3.0, 1.5, 9.0),
    Mesh3d(mesh),
    MeshMaterial3d(material),
    Transform::from_xyz(0.0, 2.0, 0.0),
    box_probes(),
  ));
}
