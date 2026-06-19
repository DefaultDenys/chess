use bevy::prelude::*;

use crate::utils::coordinate_utils;

pub fn handle_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor) = windows.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera.into_inner();

    let Ok(world) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };

    match coordinate_utils::world_to_square(world) {
        Some((file, rank)) => {
            info!("Clicked on square: ({}, {})", file, rank);
        }
        None => {
            info!("Clicked outside the board");
        }
    }
}
