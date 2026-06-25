use bevy::prelude::*;

use crate::board::game_board::Board;
use crate::chess_pieces::{BoardPosition, Piece, PieceColor};
use crate::rules;
use crate::sound::SoundAssets;
use crate::utils::coordinate_utils;

pub type PieceQuery<'w, 's> =
    Query<'w, 's, (Entity, &'static mut BoardPosition, &'static mut Transform), With<Piece>>;

/// Apply a move if it's legal. Returns `true` if the move was made.
pub fn try_move(
    commands: &mut Commands,
    board: &mut Board,
    pieces: &mut PieceQuery,
    sounds: &SoundAssets,
    from: (i32, i32),
    to: (i32, i32),
) -> bool {
    if !rules::is_legal_move(board, from, to) {
        return false;
    }

    // A capture if the target square is occupied (it's an enemy, since legality
    // already rejected capturing your own piece).
    let is_capture = board.get(to.0 as usize, to.1 as usize).is_some();

    // Capture: despawn any entity on the target square.
    for (entity, pos, _) in pieces.iter() {
        if pos.file == to.0 && pos.rank == to.1 {
            commands.entity(entity).despawn();
        }
    }

    // Update the board model.
    let moving = board.get(from.0 as usize, from.1 as usize);
    board.set(from.0 as usize, from.1 as usize, None);
    board.set(to.0 as usize, to.1 as usize, moving);

    // Move the matching sprite entity onto the target square.
    for (_, mut pos, mut transform) in pieces.iter_mut() {
        if pos.file == from.0 && pos.rank == from.1 {
            pos.file = to.0;
            pos.rank = to.1;
            transform.translation = coordinate_utils::square_to_world(to.0, to.1, 1.0);
        }
    }

    // Switch turns.
    board.turn = match board.turn {
        PieceColor::White => PieceColor::Black,
        PieceColor::Black => PieceColor::White,
    };

    // Play the matching sound (despawns itself once finished).
    let sound = if is_capture {
        sounds.capture.clone()
    } else {
        sounds.move_piece.clone()
    };
    commands.spawn((
        AudioPlayer::new(sound),
        PlaybackSettings {
            ..PlaybackSettings::DESPAWN
        },
    ));

    true
}
