pub mod height;
mod spawn;

use bevy::prelude::{App, Plugin, Startup};

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn::spawn_ocean);
  }
}
