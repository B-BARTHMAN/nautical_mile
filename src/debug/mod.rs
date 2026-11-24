mod apparent_wind_arrow;
mod buoyancy_probes;
mod grid;
mod wind_arrow;

use bevy::prelude::{App, AppGizmoBuilder, GizmoConfig, Plugin, Update, default};

use crate::debug::buoyancy_probes::ProbeGizmos;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {
    app.insert_gizmo_config(
      ProbeGizmos,
      GizmoConfig {
        depth_bias: -1.0,
        ..default()
      },
    );
    app.add_systems(
      Update,
      (
        grid::draw_grid,
        wind_arrow::draw_wind,
        apparent_wind_arrow::draw_apparent_wind,
        buoyancy_probes::draw_probes,
      ),
    );
  }
}
