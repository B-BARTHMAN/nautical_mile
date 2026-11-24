pub mod apparent;
pub mod state;

use bevy::prelude::{App, Plugin, Vec3};

use crate::wind::state::Wind;

pub struct WindPlugin;

impl Plugin for WindPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(Wind {
      velocity: Vec3::new(6.0, 0.0, 0.0),
    });
  }
}
