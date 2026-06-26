use crate::board::chess_board::BOARD_SQUARES;
use crate::chess_pieces::{Piece, PieceColor, PieceKind};
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Board {
    /// Indexed as squares[file][rank] (file = column 0..8, rank = row 0..8).
    squares: [[Option<Piece>; BOARD_SQUARES as usize]; BOARD_SQUARES as usize],
    pub turn: PieceColor,
    /// The square a pawn may capture onto by en passant, set only on the move
    /// right after an enemy pawn double-steps.
    pub en_passant: Option<(i32, i32)>,
}

impl Board {
    pub fn starting_position() -> Self {
        let mut squares = [[None; BOARD_SQUARES as usize]; BOARD_SQUARES as usize];
        const BACK_RANK: [PieceKind; BOARD_SQUARES as usize] = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];
        for file in 0..BOARD_SQUARES as usize {
            squares[file][0] = Some(Piece {
                color: PieceColor::White,
                kind: BACK_RANK[file],
                has_moved: false,
            });
            squares[file][1] = Some(Piece {
                color: PieceColor::White,
                kind: PieceKind::Pawn,
                has_moved: false,
            });
            squares[file][6] = Some(Piece {
                color: PieceColor::Black,
                kind: PieceKind::Pawn,
                has_moved: false,
            });
            squares[file][7] = Some(Piece {
                color: PieceColor::Black,
                kind: BACK_RANK[file],
                has_moved: false,
            });
        }
        Self {
            squares,
            turn: PieceColor::White,
            en_passant: None,
        }
    }

    pub fn get(&self, file: usize, rank: usize) -> Option<Piece> {
        self.squares[file][rank]
    }

    pub fn set(&mut self, file: usize, rank: usize, piece: Option<Piece>) {
        self.squares[file][rank] = piece;
    }
}
