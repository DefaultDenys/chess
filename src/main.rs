mod board;

use bevy::prelude::*;
use board::spawn_board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_board)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
