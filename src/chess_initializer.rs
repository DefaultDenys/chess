use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::system::{Commands, Res},
    math::Vec2,
    sprite::Sprite,
    transform::components::Transform,
};

use crate::{
    board::{
        chess_board::{BOARD_SQUARES, SQUARE_SIZE},
        game_board::Board,
    },
    chess_pieces::{BoardPosition, piece_sprites},
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

pub fn spawn_pieces(mut commands: Commands, asset_server: Res<AssetServer>, board: Res<Board>) {
    let texture = piece_sprites::load_pieces_texture(&asset_server);

    for file in 0..BOARD_SQUARES {
        for rank in 0..BOARD_SQUARES {
            if let Some(piece) = board.get(file as usize, rank as usize) {
                let sprite = piece_sprites::piece_sprite(piece.kind, piece.color, texture.clone());
                let transform =
                    Transform::from_translation(coordinate_utils::square_to_world(file, rank, 1.0));

                commands.spawn((piece, BoardPosition { file, rank }, sprite, transform));
            }
        }
    }
}
