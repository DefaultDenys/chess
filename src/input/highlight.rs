use bevy::prelude::*;

use crate::board::chess_board::SQUARE_SIZE;
use crate::board::game_board::Board;
use crate::rules;
use crate::utils::coordinate_utils;

use super::selection::Selection;

const HIGHLIGHT_THICKNESS: f32 = 4.0;

#[derive(Component)]
pub struct SelectionHighlight;

pub fn spawn_highlight(mut commands: Commands) {
    let color = Color::srgb(1.0, 1.0, 0.0);
    let s = SQUARE_SIZE;
    let t = HIGHLIGHT_THICKNESS;
    let edge = s / 2.0 - t / 2.0;

    commands
        .spawn((
            SelectionHighlight,
            Transform::from_xyz(0.0, 0.0, 0.5), // above board (0), below pieces (1)
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            // top / bottom edges (full width)
            parent.spawn((Sprite::from_color(color, Vec2::new(s, t)), Transform::from_xyz(0.0, edge, 0.0)));
            parent.spawn((Sprite::from_color(color, Vec2::new(s, t)), Transform::from_xyz(0.0, -edge, 0.0)));
            // left / right edges (shortened so corners aren't double-drawn)
            parent.spawn((Sprite::from_color(color, Vec2::new(t, s - 2.0 * t)), Transform::from_xyz(-edge, 0.0, 0.0)));
            parent.spawn((Sprite::from_color(color, Vec2::new(t, s - 2.0 * t)), Transform::from_xyz(edge, 0.0, 0.0)));
        });
}

pub fn update_highlight(
    selection: Res<Selection>,
    highlight: Single<(&mut Transform, &mut Visibility), With<SelectionHighlight>>,
) {
    let (mut transform, mut visibility) = highlight.into_inner();

    match selection.selected {
        Some((file, rank)) => {
            transform.translation = coordinate_utils::square_to_world(file, rank, 0.5);
            *visibility = Visibility::Visible;
        }
        None => {
            *visibility = Visibility::Hidden;
        }
    }
}

#[derive(Component)]
pub struct CheckHighlight;

pub fn spawn_check_highlight(mut commands: Commands) {
    let color = Color::srgba(0.9, 0.15, 0.15, 0.7); // red, slightly transparent
    let s = SQUARE_SIZE;
    let t = HIGHLIGHT_THICKNESS;
    let edge = s / 2.0 - t / 2.0;

    commands
        .spawn((
            CheckHighlight,
            Transform::from_xyz(0.0, 0.0, 0.6), // above selection highlight (0.5), below pieces (1)
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((Sprite::from_color(color, Vec2::new(s, t)), Transform::from_xyz(0.0, edge, 0.0)));
            parent.spawn((Sprite::from_color(color, Vec2::new(s, t)), Transform::from_xyz(0.0, -edge, 0.0)));
            parent.spawn((Sprite::from_color(color, Vec2::new(t, s - 2.0 * t)), Transform::from_xyz(-edge, 0.0, 0.0)));
            parent.spawn((Sprite::from_color(color, Vec2::new(t, s - 2.0 * t)), Transform::from_xyz(edge, 0.0, 0.0)));
        });
}

pub fn update_check_highlight(
    board: Res<Board>,
    highlight: Single<(&mut Transform, &mut Visibility), With<CheckHighlight>>,
) {
    let (mut transform, mut visibility) = highlight.into_inner();

    match rules::find_king(&board, board.turn) {
        Some((file, rank)) if rules::is_in_check(&board, board.turn) => {
            transform.translation = coordinate_utils::square_to_world(file, rank, 0.6);
            *visibility = Visibility::Visible;
        }
        _ => *visibility = Visibility::Hidden,
    }
}
