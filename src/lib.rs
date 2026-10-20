mod bitboard;
mod board;
mod coordinates;

use crate::board::*;

pub struct Engine {
    board: Board,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::standard(),
        }
    }
}
