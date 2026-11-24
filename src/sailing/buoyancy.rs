use avian3d::prelude::{Forces, Position, ReadRigidBodyForces, Rotation, WriteRigidBodyForces};
use bevy::prelude::{Component, Single, Vec3, With};

use crate::ocean::height::water_height;
use crate::sailing::boat::Boat;

const WATER_DENSITY: f32 = 1025.0;
const GRAVITY: f32 = 9.81;
const HEAVE_DAMPING: f32 = 5000.0;

pub struct Probe {
  pub offset: Vec3,
  pub volume: f32,
  pub height: f32,
}

#[derive(Component)]
pub struct BuoyancyProbes(pub Vec<Probe>);

pub fn box_probes() -> BuoyancyProbes {
  let mut probes: Vec<Probe> = Vec::new();
  for x in [-0.75, 0.75] {
    for z in [-3.0, 0.0, 3.0] {
      probes.push(Probe {
        offset: Vec3::new(x, 0.0, z),
        volume: 6.75,
        height: 1.5,
      })
    }
  }
  BuoyancyProbes(probes)
}

pub fn probe_point(position: &Position, rotation: &Rotation, probe: &Probe) -> Vec3 {
  position.0 + rotation * probe.offset
}

pub fn submerged_fraction(probe: &Probe, point: Vec3) -> f32 {
  ((water_height(point.x, point.z) - point.y) / probe.height + 0.5).clamp(0.0, 1.0)
}

pub fn lift(probe: &Probe, submerged: f32) -> f32 {
  WATER_DENSITY * GRAVITY * probe.volume * submerged
}

pub fn apply_buoyancy(
  mut boat: Single<(&Position, &Rotation, &BuoyancyProbes, Forces), With<Boat>>,
) {
  for probe in &boat.2.0 {
    let point = probe_point(boat.0, boat.1, probe);
    let submerged = submerged_fraction(probe, point);
    let lift = lift(probe, submerged);
    let damping = -boat.3.velocity_at_point(point).y * HEAVE_DAMPING * probe.volume * submerged;
    boat
      .3
      .apply_force_at_point(Vec3::Y * (lift + damping), point);
  }
}
