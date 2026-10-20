use crate::bitboard::*;

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
        Self {
            pieces: [Bitboard::empty(); 6],
            sides: [Bitboard::empty(); 2],
        }
    }
}
