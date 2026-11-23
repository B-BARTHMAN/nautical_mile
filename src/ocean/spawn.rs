use bevy::prelude::*;

pub fn spawn_ocean(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Plane3d::default().mesh().size(1000.0, 1000.0));
  let material = materials.add(StandardMaterial {
    base_color: Color::srgb(0.0, 0.42, 0.55),
    unlit: true,
    ..default()
  });
  commands.spawn((Mesh3d(mesh), MeshMaterial3d(material)));
}
