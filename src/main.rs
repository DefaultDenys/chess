mod board;
mod chess_initializer;
mod coordinate_utils;
mod pieces;

use bevy::prelude::*;
use chess_initializer::{spawn_board, spawn_pieces};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_board)
        .add_systems(Startup, spawn_pieces)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
