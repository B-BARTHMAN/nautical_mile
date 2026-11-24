mod camera;
mod debug;
mod ocean;
mod sailing;
mod sun;
mod wind;

use avian3d::PhysicsPlugins;
use bevy::prelude::{App, AppExit, DefaultPlugins};

use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::ocean::OceanPlugin;
use crate::sailing::SailingPlugin;
use crate::sun::SunPlugin;
use crate::wind::WindPlugin;

fn main() -> AppExit {
  App::new()
    .add_plugins((
      DefaultPlugins,
      PhysicsPlugins::default(),
      OceanPlugin,
      SunPlugin,
      WindPlugin,
      SailingPlugin,
      CameraPlugin,
      DebugPlugin,
    ))
    .run()
}
