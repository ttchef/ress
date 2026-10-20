use crate::coordinates::*;

use std::ops::{BitAnd, BitOrAssign, Shl};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const fn empty() -> Self {
        Bitboard(0)
    }

    pub const fn full() -> Self {
        Bitboard(!0)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn set(&mut self, square: Square) {
        *self |= square.into();
    }

    pub fn contains(&self, other: Bitboard) -> bool {
        !(*self & other).is_empty()
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl From<Square> for Bitboard {
    fn from(square: Square) -> Self {
        Bitboard(1 << (square.rank * SIZE + square.file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_emtpy() {
        let board = Bitboard::empty();
        assert!(board.is_empty());
    }

    #[test]
    fn is_empty_false() {
        let board = Bitboard(1);
        assert!(!board.is_empty());
    }

    #[test]
    fn from_square0() {
        let sq = Square::new(0, 0);
        let board: Bitboard = sq.into();
        assert_eq!(board.0, 1);
    }

    #[test]
    fn from_square1() {
        let sq = Square::new(1, 1);
        let board: Bitboard = sq.into();
        assert_eq!(board.0, 512);
    }

    #[test]
    fn set() {
        let mut board = Bitboard::empty();
        board.set(Square::new(1, 1));
        assert_eq!(board.0, 512);
    }
}
