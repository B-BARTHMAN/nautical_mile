use bevy::app::{App, Plugin, Startup, Update};
use bevy::camera::ClearColor;
use bevy::color::Color;

mod follow;
mod spawn;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)));
    app.add_systems(Startup, spawn::spawn_camera);
    app.add_systems(Update, follow::follow_boat);
  }
}
