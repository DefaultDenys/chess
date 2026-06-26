use crate::{
    board::{chess_board::BOARD_SQUARES, game_board::Board},
    chess_pieces::{PieceColor, PieceKind},
};

/// The state of the game for the side to move.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GameOutcome {
    #[default]
    Ongoing,
    Checkmate { winner: PieceColor },
    Stalemate,
}

/// Evaluate the position for whoever is to move (`board.turn`).
pub fn outcome(board: &Board) -> GameOutcome {
    let color = board.turn;
    if has_any_legal_move(board, color) {
        GameOutcome::Ongoing
    } else if is_in_check(board, color) {
        GameOutcome::Checkmate {
            winner: opposite(color),
        }
    } else {
        GameOutcome::Stalemate
    }
}

/// Does `color` have at least one legal move anywhere on the board?
pub fn has_any_legal_move(board: &Board, color: PieceColor) -> bool {
    for file in 0..BOARD_SQUARES {
        for rank in 0..BOARD_SQUARES {
            if let Some(piece) = board.get(file as usize, rank as usize)
                && piece.color == color
                && !legal_moves(board, (file, rank)).is_empty()
            {
                return true;
            }
        }
    }
    false
}

fn opposite(color: PieceColor) -> PieceColor {
    match color {
        PieceColor::White => PieceColor::Black,
        PieceColor::Black => PieceColor::White,
    }
}

/// Fully legal: a pseudo-legal move that doesn't leave your own king in check
/// (castling is handled separately, as its own special move).
pub fn is_legal_move(board: &Board, from: (i32, i32), to: (i32, i32)) -> bool {
    if is_castle_move(board, from, to) {
        return is_legal_castle(board, from, to);
    }
    if !is_pseudo_legal(board, from, to) {
        return false;
    }
    let Some(piece) = board.get(from.0 as usize, from.1 as usize) else {
        return false;
    };
    !leaves_king_in_check(board, from, to, piece.color)
}

/// Is this a castling attempt (the king moving two squares sideways)?
fn is_castle_move(board: &Board, from: (i32, i32), to: (i32, i32)) -> bool {
    matches!(board.get(from.0 as usize, from.1 as usize), Some(p) if p.kind == PieceKind::King)
        && (to.0 - from.0).abs() == 2
        && to.1 == from.1
}

/// Full castling legality: unmoved king and rook, empty squares between them,
/// and the king not in check / not passing through / not landing on an attacked square.
fn is_legal_castle(board: &Board, from: (i32, i32), to: (i32, i32)) -> bool {
    let Some(king) = board.get(from.0 as usize, from.1 as usize) else {
        return false;
    };
    if king.kind != PieceKind::King || king.has_moved {
        return false;
    }
    let color = king.color;
    let rank = from.1;
    let home_rank = match color {
        PieceColor::White => 0,
        PieceColor::Black => 7,
    };
    if rank != home_rank || from.0 != 4 {
        return false;
    }

    let kingside = to.0 > from.0;
    let (rook_file, between, king_path): (i32, &[i32], &[i32]) = if kingside {
        (7, &[5, 6], &[4, 5, 6])
    } else {
        (0, &[1, 2, 3], &[4, 3, 2])
    };

    // The rook must be present, the same color, and unmoved.
    let Some(rook) = board.get(rook_file as usize, rank as usize) else {
        return false;
    };
    if rook.kind != PieceKind::Rook || rook.color != color || rook.has_moved {
        return false;
    }

    // Squares between king and rook must be empty.
    for &file in between {
        if board.get(file as usize, rank as usize).is_some() {
            return false;
        }
    }

    // King must not be in check, nor cross / land on an attacked square.
    let enemy = opposite(color);
    for &file in king_path {
        if is_attacked(board, (file, rank), enemy) {
            return false;
        }
    }

    true
}

/// Would making `from -> to` leave `color`'s own king in check?
fn leaves_king_in_check(
    board: &Board,
    from: (i32, i32),
    to: (i32, i32),
    color: PieceColor,
) -> bool {
    let mut next = board.clone();
    let moving = next.get(from.0 as usize, from.1 as usize);
    next.set(from.0 as usize, from.1 as usize, None);

    // En passant: also remove the passed pawn (it sits beside the target square,
    // which is itself empty in this capture) so a discovered check is detected.
    if let Some(p) = moving
        && p.kind == PieceKind::Pawn
        && from.0 != to.0
        && next.get(to.0 as usize, to.1 as usize).is_none()
    {
        next.set(to.0 as usize, from.1 as usize, None);
    }

    next.set(to.0 as usize, to.1 as usize, moving);
    is_in_check(&next, color)
}

/// Is `color`'s king currently attacked?
pub fn is_in_check(board: &Board, color: PieceColor) -> bool {
    match find_king(board, color) {
        Some(king) => is_attacked(board, king, opposite(color)),
        None => false,
    }
}

/// Is `square` attacked by any piece of color `by`? Unlike how it moves, a pawn
/// attacks its two forward diagonals regardless of what's on them.
fn is_attacked(board: &Board, square: (i32, i32), by: PieceColor) -> bool {
    for file in 0..BOARD_SQUARES {
        for rank in 0..BOARD_SQUARES {
            if let Some(piece) = board.get(file as usize, rank as usize)
                && piece.color == by
                && attacks(board, (file, rank), square)
            {
                return true;
            }
        }
    }
    false
}

fn attacks(board: &Board, from: (i32, i32), to: (i32, i32)) -> bool {
    let Some(piece) = board.get(from.0 as usize, from.1 as usize) else {
        return false;
    };
    match piece.kind {
        PieceKind::Pawn => {
            let dir = match piece.color {
                PieceColor::White => 1,
                PieceColor::Black => -1,
            };
            (to.0 - from.0).abs() == 1 && to.1 - from.1 == dir
        }
        PieceKind::Knight => is_knight_move(from, to),
        PieceKind::King => is_king_move(from, to),
        PieceKind::Rook => is_rook_move(board, from, to),
        PieceKind::Bishop => is_bishop_move(board, from, to),
        PieceKind::Queen => is_rook_move(board, from, to) || is_bishop_move(board, from, to),
    }
}

pub fn find_king(board: &Board, color: PieceColor) -> Option<(i32, i32)> {
    for file in 0..BOARD_SQUARES {
        for rank in 0..BOARD_SQUARES {
            if let Some(piece) = board.get(file as usize, rank as usize)
                && piece.kind == PieceKind::King
                && piece.color == color
            {
                return Some((file, rank));
            }
        }
    }
    None
}

/// Is A from -> to B a legal pseudo-legal move? (Ignores check, castling,
/// en passant and promotion for now.) Turn ownership is checked by the caller.
pub fn is_pseudo_legal(board: &Board, from: (i32, i32), to: (i32, i32)) -> bool {
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

/// All squares the piece on `from` can legally move to right now.
pub fn legal_moves(board: &Board, from: (i32, i32)) -> Vec<(i32, i32)> {
    let mut moves = Vec::new();
    for file in 0..BOARD_SQUARES {
        for rank in 0..BOARD_SQUARES {
            if is_legal_move(board, from, (file, rank)) {
                moves.push((file, rank));
            }
        }
    }
    moves
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

    // Diagonal capture: an enemy on the target square, or en passant onto the
    // recorded square (which is itself empty in that case).
    if df.abs() == 1 && dr == dir {
        if let Some(t) = target {
            return t.color != color;
        }
        if board.en_passant == Some(to) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::game_board::Board;
    use crate::chess_pieces::{Piece, PieceColor, PieceKind};

    #[test]
    fn pawn_double_step_from_start() {
        let board = Board::starting_position();
        assert!(is_legal_move(&board, (4, 1), (4, 3))); // e2 -> e4
    }

    #[test]
    fn castling_kingside_when_path_clear() {
        let mut board = Board::starting_position();
        board.set(5, 0, None); // clear f1 bishop
        board.set(6, 0, None); // clear g1 knight
        assert!(is_legal_move(&board, (4, 0), (6, 0))); // e1 -> g1, O-O
    }

    #[test]
    fn castling_blocked_by_piece() {
        let mut board = Board::starting_position();
        board.set(6, 0, None); // clear only g1; f1 bishop still blocks
        assert!(!is_legal_move(&board, (4, 0), (6, 0)));
    }

    #[test]
    fn en_passant_capture_is_legal() {
        let mut board = Board::starting_position();
        board.set(4, 4, Some(Piece { color: PieceColor::White, kind: PieceKind::Pawn, has_moved: true }));
        board.set(3, 4, Some(Piece { color: PieceColor::Black, kind: PieceKind::Pawn, has_moved: true }));
        board.en_passant = Some((3, 5)); // black pawn just double-stepped d7-d5
        assert!(is_legal_move(&board, (4, 4), (3, 5))); // e5 x d6 e.p.
    }

    #[test]
    fn pawn_reaching_last_rank_is_legal() {
        let mut board = Board::starting_position();
        board.set(0, 6, None); // clear a black pawn out of the way
        board.set(0, 6, Some(Piece { color: PieceColor::White, kind: PieceKind::Pawn, has_moved: true }));
        board.set(0, 7, None); // clear a8 rook so the pawn can advance
        assert!(is_legal_move(&board, (0, 6), (0, 7))); // a7 -> a8 (promotes)
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
