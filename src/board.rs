use crate::bitboard::*;
use crate::coordinates::*;

use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

pub struct Board {
    pieces: [Bitboard; 6],
    sides: [Bitboard; 2],
}

impl Index<Piece> for Board {
    type Output = Bitboard;
    fn index(&self, index: Piece) -> &Self::Output {
        &self.pieces[index as usize]
    }
}

impl IndexMut<Piece> for Board {
    fn index_mut(&mut self, index: Piece) -> &mut Bitboard {
        &mut self.pieces[index as usize]
    }
}

impl Index<Side> for Board {
    type Output = Bitboard;
    fn index(&self, index: Side) -> &Self::Output {
        &self.sides[index as usize]
    }
}

impl IndexMut<Side> for Board {
    fn index_mut(&mut self, index: Side) -> &mut Bitboard {
        &mut self.sides[index as usize]
    }
}

impl Board {
    pub fn standard() -> Self {
        let mut board = Self {
            pieces: [Bitboard::empty(); 6],
            sides: [Bitboard::empty(); 2],
        };

        for file in 0..SIZE {
            board.set(Square::new(1, file), Piece::Pawn, Side::Black);
            board.set(Square::new(6, file), Piece::Pawn, Side::White);
        }

        board
    }

    fn set(&mut self, square: Square, piece: Piece, side: Side) {
        self[piece].set(square);
        self[side].set(square);
    }
}
