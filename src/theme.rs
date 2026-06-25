use bevy::prelude::*;

/// A complete visual theme: board square colors plus the piece spritesheet
/// (a 6x2 grid of `sheet_width/6` x `sheet_height/2` cells).
#[derive(Clone, Copy)]
pub struct Theme {
    pub light_square: Color,
    pub dark_square: Color,
    pub sheet_path: &'static str,
    pub sheet_width: f32,
    pub sheet_height: f32,
}

// Both kept available as switch targets; the inactive one is intentionally unused.
/// Classic: gray board + the original normalized piece set.
#[allow(dead_code)]
pub const THEME_1: Theme = Theme {
    light_square: Color::srgb(0.9, 0.9, 0.8),
    dark_square: Color::srgb(0.3, 0.3, 0.25),
    sheet_path: "pieces/chess_peaces_theme_1.png",
    sheet_width: 1536.0,
    sheet_height: 512.0,
};

/// chess.com look: green board + chess.com piece set.
#[allow(dead_code)]
pub const THEME_2: Theme = Theme {
    light_square: Color::srgb(0.922, 0.925, 0.816), // #EBECD0
    dark_square: Color::srgb(0.451, 0.584, 0.322),  // #739552
    sheet_path: "pieces/chess_peaces_theme_2.png",
    sheet_width: 900.0,
    sheet_height: 300.0,
};

/// Swap this between `THEME_1` and `THEME_2` to change the whole look.
pub const ACTIVE_THEME: Theme = THEME_2;
