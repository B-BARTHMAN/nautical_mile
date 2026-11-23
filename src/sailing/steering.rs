use crate::sailing::boat::Boat;
use bevy::prelude::{ButtonInput, KeyCode, Res, Single, Time, Transform, With};

const TURN_RATE: f32 = 0.35;
pub fn steer_boat(
  time: Res<Time>,
  keys: Res<ButtonInput<KeyCode>>,
  mut boat: Single<&mut Transform, With<Boat>>,
) {
  let mut turn = 0.0;
  if keys.pressed(KeyCode::KeyA) {
    turn += 1.0;
  } else if keys.pressed(KeyCode::KeyD) {
    turn -= 1.0;
  }

  boat.rotate_y(turn * TURN_RATE * time.delta_secs());
}
