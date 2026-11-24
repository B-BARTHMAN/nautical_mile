pub mod boat;
pub mod buoyancy;
mod spawn;

use bevy::prelude::{App, FixedUpdate, Plugin, Startup};

pub struct SailingPlugin;

impl Plugin for SailingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn::spawn_boat);
    app.add_systems(FixedUpdate, buoyancy::apply_buoyancy);
  }
}
