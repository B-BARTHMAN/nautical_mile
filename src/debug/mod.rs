mod apparent_wind_arrow;
mod grid;
mod wind_arrow;

use bevy::app::{App, Plugin, Update};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (
        grid::draw_grid,
        wind_arrow::draw_wind,
        apparent_wind_arrow::draw_apparent_wind,
      ),
    );
  }
}
