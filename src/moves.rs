use bevy::prelude::*;

use crate::board::game_board::Board;
use crate::chess_pieces::{BoardPosition, Piece, PieceColor};
use crate::utils::coordinate_utils;

pub type PieceQuery<'w, 's> =
    Query<'w, 's, (Entity, &'static mut BoardPosition, &'static mut Transform), With<Piece>>;

/// Apply a move: capture anything on the target square, update the board model,
/// move the piece entity, and switch turns.
pub fn try_move(
    commands: &mut Commands,
    board: &mut Board,
    pieces: &mut PieceQuery,
    from: (i32, i32),
    to: (i32, i32),
) {
    for (entity, pos, _) in pieces.iter() {
        if pos.file == to.0 && pos.rank == to.1 {
            commands.entity(entity).despawn();
        }
    }

    let moving = board.get(from.0 as usize, from.1 as usize);
    board.set(from.0 as usize, from.1 as usize, None);
    board.set(to.0 as usize, to.1 as usize, moving);

    for (_, mut pos, mut transform) in pieces.iter_mut() {
        if pos.file == from.0 && pos.rank == from.1 {
            pos.file = to.0;
            pos.rank = to.1;
            transform.translation = coordinate_utils::square_to_world(to.0, to.1, 1.0);
        }
    }

    board.turn = match board.turn {
        PieceColor::White => PieceColor::Black,
        PieceColor::Black => PieceColor::White,
    };
}
