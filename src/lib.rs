mod bitboard;
mod board;
mod coordinates;

pub use crate::board::*;
pub use crate::coordinates::*;

pub struct Engine {
    pub board: Board,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::standard(),
        }
    }
}
