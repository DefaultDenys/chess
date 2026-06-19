use bevy::prelude::*;

use crate::board::chess_board::SQUARE_SIZE;
use crate::chess_pieces::{PieceColor, PieceKind};

const SPRITE_SHEET_PATH: &str = "pieces/chess_pieces.png";
const IMAGE_WIDTH: f32 = 1536.0;
const IMAGE_HEIGHT: f32 = 512.0;

pub fn load_pieces_texture(asset_server: &AssetServer) -> Handle<Image> {
    asset_server.load(SPRITE_SHEET_PATH)
}

pub fn piece_sprite(kind: PieceKind, color: PieceColor, texture: Handle<Image>) -> Sprite {
    Sprite {
        image: texture,
        rect: Some(piece_rect(kind, color)),
        custom_size: Some(Vec2::splat(SQUARE_SIZE)),
        ..default()
    }
}

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

    let cell_w = IMAGE_WIDTH / 6.0;
    let cell_h = IMAGE_HEIGHT / 2.0;

    Rect {
        min: Vec2::new(col * cell_w, row * cell_h),
        max: Vec2::new((col + 1.0) * cell_w, (row + 1.0) * cell_h),
    }
}
