use glam::Vec3;

pub fn apparent_wind(true_wind: Vec3, boat_velocity: Vec3) -> Vec3 {
  true_wind - boat_velocity
}
