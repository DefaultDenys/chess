use bevy::math::primitives::{Annulus, Circle};
use bevy::prelude::*;

use crate::board::chess_board::SQUARE_SIZE;
use crate::board::game_board::Board;
use crate::rules;
use crate::utils::coordinate_utils;

use super::selection::Selection;

#[derive(Component)]
pub struct MoveIndicator;

/// Shared mesh/material handles so we don't recreate them every rebuild.
#[derive(Resource)]
pub struct IndicatorAssets {
    dot: Handle<Mesh>,
    ring: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

pub fn setup_indicators(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(IndicatorAssets {
        dot: meshes.add(Circle::new(SQUARE_SIZE * 0.16)),
        ring: meshes.add(Annulus::new(SQUARE_SIZE * 0.40, SQUARE_SIZE * 0.46)),
        material: materials.add(Color::srgba(0.1, 0.1, 0.1, 0.3)),
    });
}

pub fn update_indicators(
    mut commands: Commands,
    selection: Res<Selection>,
    board: Res<Board>,
    assets: Res<IndicatorAssets>,
    existing: Query<Entity, With<MoveIndicator>>,
) {
    // Only rebuild when the selection actually changed.
    if !selection.is_changed() {
        return;
    }

    // Clear the previous indicators.
    for entity in &existing {
        commands.entity(entity).despawn();
    }

    let Some(from) = selection.selected else {
        return;
    };

    for (file, rank) in rules::legal_moves(&board, from) {
        let is_capture = board.get(file as usize, rank as usize).is_some();
        let mesh = if is_capture {
            assets.ring.clone()
        } else {
            assets.dot.clone()
        };

        commands.spawn((
            MoveIndicator,
            Mesh2d(mesh),
            MeshMaterial2d(assets.material.clone()),
            Transform::from_translation(coordinate_utils::square_to_world(file, rank, 1.5)),
        ));
    }
}
