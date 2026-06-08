use bevy::{
    color::Color,
    ecs::system::Commands,
    math::Vec2,
    sprite::{Sprite, Text2d},
    text::TextColor,
    transform::components::Transform,
};

use crate::{
    board::{BOARD_SQUARES, SQUARE_SIZE},
    coordinate_utils,
    pieces::{BoardPosition, Piece, PieceColor, PieceKind},
};

pub fn spawn_board(mut commands: Commands) {
    let light_color = Color::srgb(0.9, 0.9, 0.8);
    let dark_color = Color::srgb(0.3, 0.3, 0.25);

    for file in 0..BOARD_SQUARES {
        for rank in 0..BOARD_SQUARES {
            let color = if (file + rank) % 2 == 0 {
                light_color
            } else {
                dark_color
            };

            let sprite = Sprite::from_color(color, Vec2::splat(SQUARE_SIZE));
            let transform =
                Transform::from_translation(coordinate_utils::square_to_world(file, rank, 0.0));

            commands.spawn((sprite, transform));
        }
    }
}

pub fn spawn_pieces(mut commands: Commands) {
    const BACK_RANK: [PieceKind; 8] = [
        PieceKind::Rook,
        PieceKind::Knight,
        PieceKind::Bishop,
        PieceKind::Queen,
        PieceKind::King,
        PieceKind::Bishop,
        PieceKind::Knight,
        PieceKind::Rook,
    ];

    for file in 0..BOARD_SQUARES {
        for rank in 0..BOARD_SQUARES {
            let piece = match rank {
                0 => Some((PieceColor::White, BACK_RANK[file as usize])),
                1 => Some((PieceColor::White, PieceKind::Pawn)),
                6 => Some((PieceColor::Black, PieceKind::Pawn)),
                7 => Some((PieceColor::Black, BACK_RANK[file as usize])),
                _ => None,
            };

            if let Some((color, kind)) = piece {
                let letter = match kind {
                    PieceKind::Pawn => "P",
                    PieceKind::Knight => "N",
                    PieceKind::Bishop => "B",
                    PieceKind::Rook => "R",
                    PieceKind::Queen => "Q",
                    PieceKind::King => "K",
                };

                let text_color = match color {
                    PieceColor::White => Color::WHITE,
                    PieceColor::Black => Color::BLACK,
                };

                let transform =
                    Transform::from_translation(coordinate_utils::square_to_world(file, rank, 1.0));

                commands.spawn((
                    Piece { color, kind },
                    BoardPosition { file, rank },
                    Text2d::new(letter),
                    TextColor(text_color),
                    transform,
                ));
            }
        }
    }
}
