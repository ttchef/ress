use crate::coordinates::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn empty() -> Self {
        Bitboard(0)
    }

    pub fn set(&mut self, square: Square) {
        self.0 |= (square.rank * SIZE + square.file) as u64;
    }
}
