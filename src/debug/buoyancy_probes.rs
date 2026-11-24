use avian3d::prelude::{Position, Rotation};
use bevy::prelude::{Color, GizmoConfigGroup, Gizmos, Reflect, Single, Vec3, With};

use crate::sailing::boat::Boat;
use crate::sailing::buoyancy::{BuoyancyProbes, lift, probe_point, submerged_fraction};

const NEWTONS_PER_METER: f32 = 20_000.0;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct ProbeGizmos;

pub fn draw_probes(
  mut gizmos: Gizmos<ProbeGizmos>,
  boat: Single<(&Position, &Rotation, &BuoyancyProbes), With<Boat>>,
) {
  let (pos, rot, probes) = boat.into_inner();
  for probe in &probes.0 {
    let point = probe_point(pos, rot, probe);
    let submerged = submerged_fraction(probe, point);
    let lift = lift(probe, submerged);

    let top = point + Vec3::Y * probe.height * 0.5;
    let bottom = point - Vec3::Y * probe.height * 0.5;
    let water = bottom + Vec3::Y * probe.height * submerged;

    gizmos.line(bottom, water, Color::srgb(0.0, 0.0, 1.0));
    gizmos.line(water, top, Color::srgb(0.5, 0.5, 0.5));
    gizmos.arrow(
      point,
      point + Vec3::Y * lift / NEWTONS_PER_METER,
      Color::srgb(0.0, 1.0, 0.0),
    );
  }
}
