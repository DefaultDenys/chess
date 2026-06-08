use bevy::prelude::*;

pub const SQUARE_SIZE: f32 = 64.0;

pub fn spawn_board(mut commands: Commands) {
    let board_squares: i32 = 8;

    let light_color = Color::srgb(0.9, 0.9, 0.8);
    let dark_color = Color::srgb(0.3, 0.3, 0.25);

    for row in 0..board_squares {
        for column in 0..board_squares {
            let color = if (row + column) % 2 == 0 {
                light_color
            } else {
                dark_color
            };

            let center_offset = board_squares as f32 / 2.0 - 0.5;
            let x = (row as f32 - center_offset) * SQUARE_SIZE;
            let y = (column as f32 - center_offset) * SQUARE_SIZE;
            let z = 0.0;

            let sprite = Sprite::from_color(color, Vec2::splat(SQUARE_SIZE));
            let transform = Transform::from_xyz(x, y, z);

            commands.spawn((sprite, transform));
        }
    }
}