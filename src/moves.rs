use bevy::prelude::*;

use crate::board::game_board::Board;
use crate::chess_pieces::{BoardPosition, Piece, PieceColor, PieceKind};
use crate::promotion::{Pending, Promotion};
use crate::rules;
use crate::sound::SoundAssets;
use crate::utils::coordinate_utils;

pub type PieceQuery<'w, 's> =
    Query<'w, 's, (Entity, &'static mut BoardPosition, &'static mut Transform), With<Piece>>;

/// Apply a move if it's legal. Returns `true` if the move was made. Handles the
/// special moves: castling, en passant, and promotion (which pauses for the
/// player's choice instead of switching turns immediately).
pub fn try_move(
    commands: &mut Commands,
    board: &mut Board,
    pieces: &mut PieceQuery,
    sounds: &SoundAssets,
    promotion: &mut Promotion,
    from: (i32, i32),
    to: (i32, i32),
) -> bool {
    if !rules::is_legal_move(board, from, to) {
        return false;
    }

    let mover = board
        .get(from.0 as usize, from.1 as usize)
        .expect("a legal move always has a piece on `from`");

    let normal_capture = board.get(to.0 as usize, to.1 as usize).is_some();
    let is_pawn = mover.kind == PieceKind::Pawn;
    let is_castle = mover.kind == PieceKind::King && (to.0 - from.0).abs() == 2;
    // A diagonal pawn move onto an empty square can only be en passant.
    let is_en_passant = is_pawn && from.0 != to.0 && !normal_capture;
    let last_rank = match mover.color {
        PieceColor::White => 7,
        PieceColor::Black => 0,
    };
    let is_promotion = is_pawn && to.1 == last_rank;

    // En passant removes the passed pawn, which sits beside the target square
    // (same file as `to`, same rank as `from`).
    if is_en_passant {
        let captured = (to.0, from.1);
        despawn_at(commands, pieces, captured);
        board.set(captured.0 as usize, captured.1 as usize, None);
    }

    // Normal capture removes whatever was on the target square.
    if normal_capture {
        despawn_at(commands, pieces, to);
    }

    // Move the piece in the model and on screen. The kind is unchanged for now;
    // a promotion is finalized once the player picks a piece.
    board.set(from.0 as usize, from.1 as usize, None);
    let landed = Piece {
        has_moved: true,
        ..mover
    };
    board.set(to.0 as usize, to.1 as usize, Some(landed));
    move_entity(pieces, from, to);

    // Castling also moves the rook to the other side of the king.
    if is_castle {
        let rank = from.1;
        let (rook_from, rook_to) = if to.0 > from.0 {
            ((7, rank), (5, rank)) // kingside: h -> f
        } else {
            ((0, rank), (3, rank)) // queenside: a -> d
        };
        if let Some(mut rook) = board.get(rook_from.0 as usize, rook_from.1 as usize) {
            rook.has_moved = true;
            board.set(rook_from.0 as usize, rook_from.1 as usize, None);
            board.set(rook_to.0 as usize, rook_to.1 as usize, Some(rook));
            move_entity(pieces, rook_from, rook_to);
        }
    }

    // Record the en-passant target square, set only right after a double step.
    board.en_passant = if is_pawn && (to.1 - from.1).abs() == 2 {
        Some((from.0, (from.1 + to.1) / 2))
    } else {
        None
    };

    // A promotion pauses for the player's choice; otherwise hand over the turn.
    if is_promotion {
        promotion.pending = Some(Pending {
            square: to,
            color: mover.color,
        });
    } else {
        board.turn = match board.turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
    }

    // Play the matching sound (despawns itself once finished).
    let captured = normal_capture || is_en_passant;
    let sound = if captured {
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

/// Despawn the piece entity sitting on `square`, if any.
fn despawn_at(commands: &mut Commands, pieces: &PieceQuery, square: (i32, i32)) {
    for (entity, pos, _) in pieces.iter() {
        if pos.file == square.0 && pos.rank == square.1 {
            commands.entity(entity).despawn();
        }
    }
}

/// Move the piece entity from `from` to `to` (board position + on-screen transform).
fn move_entity(pieces: &mut PieceQuery, from: (i32, i32), to: (i32, i32)) {
    for (_, mut pos, mut transform) in pieces.iter_mut() {
        if pos.file == from.0 && pos.rank == from.1 {
            pos.file = to.0;
            pos.rank = to.1;
            transform.translation = coordinate_utils::square_to_world(to.0, to.1, 1.0);
        }
    }
}
