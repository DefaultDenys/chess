use bevy::prelude::*;

use crate::board::chess_board::SQUARE_SIZE;
use crate::board::game_board::Board;
use crate::chess_pieces::PieceColor;
use crate::rules::{self, GameOutcome};

#[derive(Resource, Default)]
pub struct GameState {
    pub outcome: GameOutcome,
}

impl GameState {
    pub fn is_over(&self) -> bool {
        self.outcome != GameOutcome::Ongoing
    }
}

#[derive(Component)]
pub struct GameBanner;

pub fn spawn_banner(mut commands: Commands) {
    // World-space text just above the board's top edge (the board spans
    // -4..4 squares vertically from the center), so it scales with the board.
    let board_top = 4.0 * SQUARE_SIZE;
    commands.spawn((
        GameBanner,
        Text2d::new(""),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.85, 0.1, 0.1)),
        Transform::from_xyz(0.0, board_top + 16.0, 10.0),
    ));
}

/// After each move (when the board changes), recompute the outcome and update
/// the banner text. Empty text while the game is ongoing.
pub fn update_game_state(
    board: Res<Board>,
    mut game_state: ResMut<GameState>,
    banner: Single<&mut Text2d, With<GameBanner>>,
) {
    if !board.is_changed() {
        return;
    }

    let outcome = rules::outcome(&board);
    game_state.outcome = outcome;

    let message = match outcome {
        GameOutcome::Ongoing => String::new(),
        GameOutcome::Checkmate { winner } => {
            let name = match winner {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            };
            format!("Checkmate - {name} wins")
        }
        GameOutcome::Stalemate => "Stalemate - draw".to_string(),
    };

    let mut text = banner.into_inner();
    *text = Text2d::new(message);
}
