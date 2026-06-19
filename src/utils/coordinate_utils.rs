use bevy::math::{Vec2, Vec3};

use crate::board::chess_board::{BOARD_SQUARES, SQUARE_SIZE};

pub fn square_to_world(file: i32, rank: i32, z: f32) -> Vec3 {
    let center_offset = 8.0_f32 / 2.0 - 0.5;
    let x = (file as f32 - center_offset) * SQUARE_SIZE;
    let y = (rank as f32 - center_offset) * SQUARE_SIZE;
    Vec3::new(x, y, z)
}

pub fn world_to_square(position: Vec2) -> Option<(i32, i32)> {
    let center_offset = 8.0_f32 / 2.0 - 0.5;
    let file = (position.x / SQUARE_SIZE + center_offset).round() as i32;
    let rank = (position.y / SQUARE_SIZE + center_offset).round() as i32;

    if (0..BOARD_SQUARES).contains(&file) && (0..BOARD_SQUARES).contains(&rank) {
        Some((file, rank))
    } else {
        None
    }
}
