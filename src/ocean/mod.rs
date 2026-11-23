use bevy::app::{App, Plugin, Startup};

mod spawn;
pub struct OceanPlugin;

impl Plugin for OceanPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn::spawn_ocean);
  }
}
