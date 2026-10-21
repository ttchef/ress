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
        if side != self.to_move {
            return Bitboard::empty();
        }
        self.board.get_pseudo_legal_moves(piece, side, square)
    }

    pub fn make_move(&mut self, piece: Piece, from: Bitboard, to: Bitboard) {
        self.board.make_move(piece, self.to_move, from, to);
        self.to_move = self.to_move.other();
    }
}
