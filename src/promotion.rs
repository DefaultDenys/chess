use bevy::prelude::*;

use crate::board::chess_board::SQUARE_SIZE;
use crate::chess_pieces::{PieceColor, PieceKind, piece_sprites};
use crate::utils::coordinate_utils;

/// The pieces a pawn may promote to, in display order from the promotion square.
const OPTIONS: [PieceKind; 4] = [
    PieceKind::Queen,
    PieceKind::Rook,
    PieceKind::Bishop,
    PieceKind::Knight,
];

#[derive(Clone, Copy)]
pub struct Pending {
    pub square: (i32, i32),
    pub color: PieceColor,
}

/// When `pending` is set, the game is waiting for the player to choose which
/// piece a pawn promotes to.
#[derive(Resource, Default)]
pub struct Promotion {
    pub pending: Option<Pending>,
}

#[derive(Component)]
pub struct PromotionOption;

/// Direction the option list extends from the promotion square (into the board).
fn list_dir(color: PieceColor) -> i32 {
    match color {
        PieceColor::White => -1, // promotes on rank 7, options descend
        PieceColor::Black => 1,  // promotes on rank 0, options ascend
    }
}

/// Which promotion piece (if any) the player clicked, for the pending promotion.
pub fn clicked_choice(pending: Pending, square: (i32, i32)) -> Option<PieceKind> {
    if square.0 != pending.square.0 {
        return None;
    }
    let index = (square.1 - pending.square.1) * list_dir(pending.color);
    if (0..OPTIONS.len() as i32).contains(&index) {
        Some(OPTIONS[index as usize])
    } else {
        None
    }
}

/// Show the four choices (a backdrop + the piece on each square) while a
/// promotion is pending, and clear them once it's resolved.
pub fn update_promotion_ui(
    mut commands: Commands,
    promotion: Res<Promotion>,
    asset_server: Res<AssetServer>,
    existing: Query<Entity, With<PromotionOption>>,
) {
    if !promotion.is_changed() {
        return;
    }

    for entity in &existing {
        commands.entity(entity).despawn();
    }

    let Some(pending) = promotion.pending else {
        return;
    };

    let dir = list_dir(pending.color);
    let texture = piece_sprites::load_pieces_texture(&asset_server);
    for (i, kind) in OPTIONS.iter().enumerate() {
        let rank = pending.square.1 + dir * i as i32;
        let center = coordinate_utils::square_to_world(pending.square.0, rank, 0.0);

        // Light backdrop so the choices stand out over the board.
        commands.spawn((
            PromotionOption,
            Sprite::from_color(Color::srgb(0.95, 0.95, 0.95), Vec2::splat(SQUARE_SIZE)),
            Transform::from_xyz(center.x, center.y, 3.0),
        ));
        commands.spawn((
            PromotionOption,
            piece_sprites::piece_sprite(*kind, pending.color, texture.clone()),
            Transform::from_xyz(center.x, center.y, 3.1),
        ));
    }
}
