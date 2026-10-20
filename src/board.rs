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

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::Knight,
    ];
}

impl Side {
    pub const ALL: [Side; 2] = [Side::White, Side::Black];
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

        const BACK_RANK: [Piece; 8] = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::Knight,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];

        for file in 0..SIZE {
            board.set(Square::new(0, file), BACK_RANK[file as usize], Side::Black);
            board.set(Square::new(1, file), Piece::Pawn, Side::Black);
            board.set(Square::new(6, file), Piece::Pawn, Side::White);
            board.set(Square::new(7, file), BACK_RANK[file as usize], Side::White);
        }

        board
    }

    pub fn set(&mut self, square: Square, piece: Piece, side: Side) {
        self[piece].set(square);
        self[side].set(square);
    }

    pub fn get(&self, square: Square) -> Option<(Piece, Side)> {
        let mask: Bitboard = square.into();

        let piece = Piece::ALL.iter().find(|&&p| self[p].contains(mask))?;
        let side = Side::ALL.iter().find(|&&s| self[s].contains(mask))?;

        Some((*piece, *side))
    }

    pub fn get_moves(&self, piece: Piece, side: Side, square: Option<Square>) -> Bitboard {
        let square_mask = match square {
            Some(sq) => sq.into(),
            None => Bitboard::full(),
        };

        let piece_board = self[piece];
        let side_board = self[side];

        match piece {
            _ => piece_board << 8,
        }
    }
}
