mod follow;
mod spawn;

use bevy::prelude::{App, ClearColor, Color, Plugin, Startup, Update};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)));
    app.add_systems(Startup, spawn::spawn_camera);
    app.add_systems(Update, follow::follow_boat);
  }
}
