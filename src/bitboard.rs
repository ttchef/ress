use crate::coordinates::*;

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard(pub u64);

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

    pub fn offset(&self, dir: Compass, steps: u8) -> Bitboard {
        // TODO: Branchless version with rotate left and wrap mask
        let offset = dir as i32 * steps as i32;
        if offset > 0 {
            Bitboard(self.0 << offset)
        } else if offset < 0 {
            Bitboard(self.0 >> offset.abs())
        } else {
            Bitboard::empty()
        }
    }

    // TODO: Replace by something a little more performant like
    // Kogge-Stone or even a lookup with magic bitboards
    pub fn dumb7fill(&self, dir: Compass, pro: Bitboard) -> Bitboard {
        // TODO: Wrap arounds
        let mut g = *self;
        let mut flood = Bitboard(0);
        while !g.is_empty() {
            flood |= g;
            g = g.offset(dir, 1) & pro;
        }
        flood
    }

    /// Zero indexed
    pub const fn rank(n: u32) -> Bitboard {
        Bitboard(0xff << (n * 8))
    }

    /// Zero indexed
    pub const fn file(n: u32) -> Bitboard {
        Bitboard(0x0101010101010101 << n)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 >> rhs)
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

    #[test]
    fn rank() {
        let rank4 = Bitboard::rank(4);
        assert_eq!(rank4, Bitboard(0x000000ff00000000));
    }

    #[test]
    fn file() {
        let file8 = Bitboard::file(7);
        assert_eq!(file8, Bitboard(0x8080808080808080));
    }

    #[test]
    fn offset0() {
        let bb = Bitboard(0b001);
        assert_eq!(bb.offset(Compass::East, 2), Bitboard(0b100))
    }

    #[test]
    fn offset1() {
        let bb = Bitboard(0b0000_0010_0000_0000);
        assert_eq!(
            bb.offset(Compass::South, 1),
            Bitboard(0b0000_0000_0000_0010)
        );
    }

    #[test]
    fn offset2() {
        let bb = Bitboard(0b0000_0010_0000_0000);
        assert_eq!(
            bb.offset(Compass::North, 1),
            Bitboard(0b0000_0010_0000_0000_0000_0000)
        );
    }

    #[test]
    fn offset3() {
        let bb = Bitboard(
            0b0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(bb.offset(Compass::North, 1), Bitboard(0));
    }
}
