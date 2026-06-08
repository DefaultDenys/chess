use bevy::math::Vec3;

use crate::board::SQUARE_SIZE;

pub fn square_to_world(file: i32, rank: i32, z: f32) -> Vec3 {
    let center_offset = 8.0_f32 / 2.0 - 0.5;
    let x = (file as f32 - center_offset) * SQUARE_SIZE;
    let y = (rank as f32 - center_offset) * SQUARE_SIZE;
    Vec3::new(x, y, z)
}
