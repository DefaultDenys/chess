use bevy::prelude::*;

use crate::board::game_board::Board;
use crate::chess_pieces::{BoardPosition, Piece, PieceColor, piece_sprites};
use crate::game::GameState;
use crate::moves::{PieceQuery, try_move};
use crate::promotion::{self, Promotion};
use crate::sound::SoundAssets;
use crate::utils::coordinate_utils;

use super::selection::Selection;

/// How far (in world units) the cursor must move after pressing before it
/// counts as a drag rather than a click. Below this, the piece stays put.
const DRAG_THRESHOLD: f32 = 8.0;

pub fn handle_mouse(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut board: ResMut<Board>,
    mut selection: ResMut<Selection>,
    mut pieces: PieceQuery,
    sounds: Res<SoundAssets>,
    asset_server: Res<AssetServer>,
    mut promotion: ResMut<Promotion>,
    game_state: Res<GameState>,
) {
    if game_state.is_over() {
        return;
    }

    let window = window.into_inner();
    let (camera, camera_transform) = camera.into_inner();

    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(world) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };
    let square = coordinate_utils::world_to_square(world);

    // --- Promotion: while a choice is pending, a click picks the piece and
    // normal board interaction is suspended. ---
    if let Some(pending) = promotion.pending {
        if mouse.just_pressed(MouseButton::Left)
            && let Some(clicked) = square
            && let Some(kind) = promotion::clicked_choice(pending, clicked)
        {
            let piece = Piece {
                color: pending.color,
                kind,
                has_moved: true,
            };
            board.set(pending.square.0 as usize, pending.square.1 as usize, Some(piece));

            // Replace the pawn entity with the chosen piece.
            for (entity, pos, _) in pieces.iter() {
                if pos.file == pending.square.0 && pos.rank == pending.square.1 {
                    commands.entity(entity).despawn();
                }
            }
            let texture = piece_sprites::load_pieces_texture(&asset_server);
            commands.spawn((
                piece,
                BoardPosition {
                    file: pending.square.0,
                    rank: pending.square.1,
                },
                piece_sprites::piece_sprite(kind, pending.color, texture),
                Transform::from_translation(coordinate_utils::square_to_world(
                    pending.square.0,
                    pending.square.1,
                    1.0,
                )),
            ));

            board.turn = match board.turn {
                PieceColor::White => PieceColor::Black,
                PieceColor::Black => PieceColor::White,
            };
            promotion.pending = None;
        }
        return;
    }

    // --- Press: select your own piece, or try to complete a click-to-move ---
    if mouse.just_pressed(MouseButton::Left) {
        match square {
            None => selection.clear(),
            Some((file, rank)) => {
                if is_own_piece(&board, file, rank) {
                    selection.selected = Some((file, rank));
                    selection.press_origin = Some(world);
                    selection.dragging = false;
                } else if let Some(from) = selection.selected {
                    // Move only if legal; otherwise keep the selection so the
                    // player can pick a different target.
                    if try_move(
                        &mut commands,
                        &mut board,
                        &mut pieces,
                        &sounds,
                        &mut promotion,
                        from,
                        (file, rank),
                    ) {
                        selection.clear();
                    }
                } else {
                    selection.clear();
                }
            }
        }
    }

    // --- Hold: promote to a drag once the cursor moves far enough, then follow it ---
    if mouse.pressed(MouseButton::Left) {
        if let (Some((file, rank)), Some(origin)) = (selection.selected, selection.press_origin) {
            if !selection.dragging && world.distance(origin) > DRAG_THRESHOLD {
                selection.dragging = true;
            }

            if selection.dragging {
                for (_, pos, mut transform) in &mut pieces {
                    if pos.file == file && pos.rank == rank {
                        transform.translation = world.extend(2.0); // z=2 -> on top while dragging
                    }
                }
            }
        }
    }

    // --- Release: drop the dragged piece (move if legal, else snap back) ---
    if mouse.just_released(MouseButton::Left) {
        if let Some((from_file, from_rank)) = selection.selected {
            if selection.dragging {
                let moved = match square {
                    Some((file, rank)) => try_move(
                        &mut commands,
                        &mut board,
                        &mut pieces,
                        &sounds,
                        &mut promotion,
                        (from_file, from_rank),
                        (file, rank),
                    ),
                    None => false,
                };

                if moved {
                    selection.clear();
                } else {
                    snap_to_square(&mut pieces, from_file, from_rank);
                    selection.press_origin = None;
                    selection.dragging = false;
                }
            } else {
                selection.press_origin = None;
            }
        }
    }
}

fn is_own_piece(board: &Board, file: i32, rank: i32) -> bool {
    board
        .get(file as usize, rank as usize)
        .map(|piece| piece.color == board.turn)
        .unwrap_or(false)
}

fn snap_to_square(pieces: &mut PieceQuery, file: i32, rank: i32) {
    for (_, pos, mut transform) in pieces.iter_mut() {
        if pos.file == file && pos.rank == rank {
            transform.translation = coordinate_utils::square_to_world(file, rank, 1.0);
        }
    }
}
