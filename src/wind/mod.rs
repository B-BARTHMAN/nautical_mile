pub mod state;

use crate::wind::state::Wind;
use bevy::app::App;
use bevy::prelude::{Plugin, Vec3};

pub struct WindPlugin;

impl Plugin for WindPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(Wind {
      velocity: Vec3::new(6.0, 0.0, 0.0),
    });
  }
}
