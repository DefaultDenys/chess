use bevy::prelude::*;

#[derive(Resource)]
pub struct SoundAssets {
    pub move_piece: Handle<AudioSource>,
    pub capture: Handle<AudioSource>,
}

pub fn setup_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundAssets {
        move_piece: asset_server.load("sounds/move.wav"),
        capture: asset_server.load("sounds/capture.wav"),
    });
}
