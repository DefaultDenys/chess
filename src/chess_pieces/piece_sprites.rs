use bevy::prelude::*;

use crate::board::chess_board::SQUARE_SIZE;
use crate::chess_pieces::{PieceColor, PieceKind};
use crate::theme::ACTIVE_THEME;

pub fn load_pieces_texture(asset_server: &AssetServer) -> Handle<Image> {
    asset_server.load(ACTIVE_THEME.sheet_path)
}

pub fn piece_sprite(kind: PieceKind, color: PieceColor, texture: Handle<Image>) -> Sprite {
    Sprite {
        image: texture,
        rect: Some(piece_rect(kind, color)),
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..default()
    }
}

/// The sheet is a 6x2 grid: columns King, Queen, Bishop, Knight, Rook, Pawn;
/// row 0 white, row 1 black.
fn piece_rect(kind: PieceKind, color: PieceColor) -> Rect {
    let col = match kind {
        PieceKind::King => 0.0,
        PieceKind::Queen => 1.0,
        PieceKind::Bishop => 2.0,
        PieceKind::Knight => 3.0,
        PieceKind::Rook => 4.0,
        PieceKind::Pawn => 5.0,
    };

    let row = match color {
        PieceColor::White => 0.0,
        PieceColor::Black => 1.0,
    };

    let cell_w = ACTIVE_THEME.sheet_width / 6.0;
    let cell_h = ACTIVE_THEME.sheet_height / 2.0;

    Rect {
        min: Vec2::new(col * cell_w, row * cell_h),
        max: Vec2::new((col + 1.0) * cell_w, (row + 1.0) * cell_h),
    }
}
