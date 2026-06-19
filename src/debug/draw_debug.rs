use bevy::{
    color::Color,
    ecs::{query::With, system::Query},
    gizmos::gizmos::Gizmos,
    math::{Isometry2d, Vec2},
    transform::components::Transform,
};

use crate::{board::chess_board::SQUARE_SIZE, chess_pieces::Piece};

pub fn debug_piece_bounds(mut gizmos: Gizmos, pieces: Query<&Transform, With<Piece>>) {
    for transform in &pieces {
        gizmos.rect_2d(
            Isometry2d::from_translation(transform.translation.truncate()),
            Vec2::splat(SQUARE_SIZE),
            Color::linear_rgb(1.0, 0.0, 0.0),
        );
    }
}
