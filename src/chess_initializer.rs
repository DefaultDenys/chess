use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::system::{Commands, Res},
    math::Vec2,
    sprite::Sprite,
    transform::components::Transform,
};

use crate::{
    board::{BOARD_SQUARES, SQUARE_SIZE},
    chess_pieces::{BoardPosition, Piece, PieceColor, PieceKind, piece_sprites},
    utils::coordinate_utils,
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

pub fn spawn_pieces(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture =  piece_sprites::load_pieces_texture(&asset_server);

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
                let sprite = piece_sprites::piece_sprite(kind, color, texture.clone());
                let transform =
                    Transform::from_translation(coordinate_utils::square_to_world(file, rank, 1.0));

                commands.spawn((
                    Piece { color, kind },
                    BoardPosition { file, rank },
                    sprite,
                    transform,
                ));
            }
        }
    }
}
