mod spawn;

use bevy::prelude::{App, Plugin, Startup};

pub struct SunPlugin;

impl Plugin for SunPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn::spawn_sun);
  }
}
