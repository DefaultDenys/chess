use bevy::ecs::component::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardPosition {
    pub file: i32,
    pub rank: i32,
}
