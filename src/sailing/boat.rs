use bevy::prelude::Component;
use glam::Vec3;

#[derive(Component, Default)]
pub struct Boat {
  pub velocity: Vec3,
}
