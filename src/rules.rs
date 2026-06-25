use crate::{
    board::{chess_board::BOARD_SQUARES, game_board::Board},
    chess_pieces::{PieceColor, PieceKind},
};

// Is A from -> to B a legal pseudo-legal move? (Ignores check, castling,
// en passant and promotion for now.) Turn ownership is checked by the caller.
pub fn is_legal_move(board: &Board, from: (i32, i32), to: (i32, i32)) -> bool {
    if from == to || !on_board(to.0, to.1) {
        return false; // same square or off board
    }

    let Some(piece) = board.get(from.0 as usize, from.1 as usize) else {
        return false; // nothing to move
    };

    // Can't capture your own piece.
    if let Some(target) = board.get(to.0 as usize, to.1 as usize)
        && target.color == piece.color
    {
        return false;
    }

    match piece.kind {
        PieceKind::Knight => is_knight_move(from, to),
        PieceKind::King => is_king_move(from, to),
        PieceKind::Rook => is_rook_move(board, from, to),
        PieceKind::Bishop => is_bishop_move(board, from, to),
        PieceKind::Queen => is_rook_move(board, from, to) || is_bishop_move(board, from, to),
        PieceKind::Pawn => is_pawn_move(board, piece.color, from, to),
    }
}

fn on_board(file: i32, rank: i32) -> bool {
    (0..BOARD_SQUARES).contains(&file) && (0..BOARD_SQUARES).contains(&rank)
}

/// True if every square strictly between `from` and `to` is empty.
fn path_clear(board: &Board, from: (i32, i32), to: (i32, i32)) -> bool {
    let step_f = (to.0 - from.0).signum();
    let step_r = (to.1 - from.1).signum();

    let mut file = from.0 + step_f;
    let mut rank = from.1 + step_r;
    while (file, rank) != to {
        if board.get(file as usize, rank as usize).is_some() {
            return false;
        }
        file += step_f;
        rank += step_r;
    }
    true
}

fn is_knight_move(from: (i32, i32), to: (i32, i32)) -> bool {
    let df = (to.0 - from.0).abs();
    let dr = (to.1 - from.1).abs();
    (df == 1 && dr == 2) || (df == 2 && dr == 1)
}

fn is_king_move(from: (i32, i32), to: (i32, i32)) -> bool {
    let df = (to.0 - from.0).abs();
    let dr = (to.1 - from.1).abs();
    df <= 1 && dr <= 1
}

fn is_rook_move(board: &Board, from: (i32, i32), to: (i32, i32)) -> bool {
    if from.0 != to.0 && from.1 != to.1 {
        return false; // not a straight line
    }
    path_clear(board, from, to)
}

fn is_bishop_move(board: &Board, from: (i32, i32), to: (i32, i32)) -> bool {
    if (to.0 - from.0).abs() != (to.1 - from.1).abs() {
        return false; // not a diagonal
    }
    path_clear(board, from, to)
}

fn is_pawn_move(board: &Board, color: PieceColor, from: (i32, i32), to: (i32, i32)) -> bool {
    let dir = match color {
        PieceColor::White => 1,
        PieceColor::Black => -1,
    };
    let start_rank = match color {
        PieceColor::White => 1,
        PieceColor::Black => 6,
    };

    let df = to.0 - from.0;
    let dr = to.1 - from.1;
    let target = board.get(to.0 as usize, to.1 as usize);

    // One square forward (must be empty).
    if df == 0 && dr == dir && target.is_none() {
        return true;
    }

    // Two squares forward from the start (both squares empty).
    if df == 0
        && dr == 2 * dir
        && from.1 == start_rank
        && target.is_none()
        && board
            .get(from.0 as usize, (from.1 + dir) as usize)
            .is_none()
    {
        return true;
    }

    // Diagonal capture (an enemy must be there).
    if df.abs() == 1 && dr == dir {
        if let Some(t) = target {
            return t.color != color;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::game_board::Board;

    #[test]
    fn pawn_double_step_from_start() {
        let board = Board::starting_position();
        assert!(is_legal_move(&board, (4, 1), (4, 3))); // e2 -> e4
    }

    #[test]
    fn knight_jumps_out_over_pawns() {
        let board = Board::starting_position();
        assert!(is_legal_move(&board, (1, 0), (2, 2))); // Nb1 -> c3
    }

    #[test]
    fn rook_blocked_by_own_pawn() {
        let board = Board::starting_position();
        assert!(!is_legal_move(&board, (0, 0), (0, 3))); // Ra1 can't jump its pawn
    }

    #[test]
    fn cannot_capture_own_piece() {
        let board = Board::starting_position();
        assert!(!is_legal_move(&board, (0, 0), (0, 1))); // Ra1 -> a2 (own pawn)
    }
}
