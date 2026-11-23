use crate::sailing::boat::Boat;
use bevy::prelude::{Res, Single, Transform};
use bevy::time::Time;

const SPEED: f32 = 2.5;

pub fn move_boat(time: Res<Time>, boat: Single<(&mut Transform, &mut Boat)>) {
  let (mut transform, mut boat) = boat.into_inner();
  boat.velocity = transform.forward() * SPEED;
  transform.translation += boat.velocity * time.delta_secs();
}
