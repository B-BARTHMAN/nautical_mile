use bevy::prelude::{Resource, Vec3};

#[derive(Resource)]
pub struct Wind {
  pub velocity: Vec3,
}
