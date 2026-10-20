mod bitboard;
mod board;
mod coordinates;

use crate::bitboard::Bitboard;
pub use crate::board::*;
pub use crate::coordinates::*;

pub struct Engine {
    pub board: Board,
    pub to_move: Side,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::standard(),
            to_move: Side::White,
        }
    }

    pub fn get_moves(&self, piece: Piece, side: Side, square: Option<Square>) -> Bitboard {
        self.board
            .get_pseudo_legal_moves(piece, side, square, self.to_move)
    }
}
