mod camera;
mod debug;
mod ocean;
mod physics;
mod sailing;
mod sun;
mod wind;

use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::ocean::OceanPlugin;
use crate::sailing::SailingPlugin;
use crate::sun::SunPlugin;
use crate::wind::WindPlugin;
use bevy::prelude::*;

fn main() -> AppExit {
  App::new()
    .add_plugins((
      DefaultPlugins,
      CameraPlugin,
      OceanPlugin,
      SunPlugin,
      SailingPlugin,
      DebugPlugin,
      WindPlugin,
    ))
    .run()
}
