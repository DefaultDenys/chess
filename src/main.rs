mod board;
mod chess_initializer;
mod chess_pieces;
mod debug;
mod game;
mod input;
mod moves;
mod rules;
mod sound;
mod theme;
mod utils;

use bevy::{camera::ScalingMode, prelude::*};
use chess_initializer::{spawn_board, spawn_pieces};

use crate::{board::game_board::Board, debug::draw_debug::debug_piece_bounds};

const DEBUG_MODE: bool = false;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_board)
        .add_systems(Startup, spawn_pieces)
        .insert_resource(Board::starting_position())
        .add_systems(Startup, sound::setup_sounds)
        .add_systems(Startup, input::spawn_highlight)
        .add_systems(Startup, input::spawn_check_highlight)
        .add_systems(Startup, input::setup_indicators)
        .add_systems(Update, input::update_highlight)
        .add_systems(Update, input::update_check_highlight)
        .add_systems(Update, input::update_indicators)
        .add_systems(Update, input::handle_mouse)
        .add_systems(Startup, game::spawn_banner)
        .add_systems(Update, game::update_game_state)
        .init_resource::<input::Selection>()
        .init_resource::<game::GameState>();

    if DEBUG_MODE {
        app.add_systems(Update, debug_piece_bounds);
    }

    app.run();
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
