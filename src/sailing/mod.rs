pub mod boat;
mod movement;
mod spawn;
mod steering;

use bevy::app::{App, FixedUpdate, Plugin, Startup};
use bevy::prelude::IntoScheduleConfigs;

pub struct SailingPlugin;

impl Plugin for SailingPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn::spawn_boat);
    app.add_systems(
      FixedUpdate,
      (steering::steer_boat, movement::move_boat).chain(),
    );
  }
}
