use crate::piece::Piece;

pub const SIZE: usize = 8;

#[derive(Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    // Returns true if bit on this square is set
    pub fn square(&self, row: usize, col: usize) -> bool {
        self.0 & (1 << (row * SIZE + col)) != 0
    }
}

pub struct Board {
    pieces: [Bitboard; 6],
    team: [Bitboard; 2],
}

impl Board {
    pub fn standard() -> Self {
        Self {
            pieces: [Bitboard(0); 6],
            team: [Bitboard(0); 2],
        }
    }

    pub fn get_square(row: usize, col: usize) -> Option<Piece> {
        None
    }
}

#[test]
fn bitboard_square() {
    let board = Bitboard(1 << (2 * SIZE + 4));
    assert!(board.square(2, 4));
}
