mod board;
mod chess_initializer;
mod chess_pieces;
mod debug;
mod input;
mod moves;
mod rules;
mod utils;

use bevy::{camera::ScalingMode, prelude::*};
use chess_initializer::{spawn_board, spawn_pieces};

use crate::{board::game_board::Board, debug::draw_debug::debug_piece_bounds};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_board)
        .add_systems(Startup, spawn_pieces)
        // .add_systems(Update, debug_piece_bounds) //Debug system to visualize piece bounds
        .insert_resource(Board::starting_position())
        .add_systems(Startup, input::spawn_highlight)
        .add_systems(Update, input::update_highlight)
        .add_systems(Update, input::handle_mouse)
        .init_resource::<input::Selection>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 576.0,
                min_height: 576.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
