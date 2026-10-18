const UNIQUE_PIECES: usize = 6;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub const fn other(&self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub kind: Kind,
    pub side: Side,
}

impl Piece {
    pub const fn new(kind: Kind, side: Side) -> Self {
        Self { kind, side }
    }

    pub fn index(&self) -> usize {
        self.kind as usize + UNIQUE_PIECES * self.side as usize
    }

    pub fn from_index(index: usize) -> Self {
        let side = index / UNIQUE_PIECES;
        let kind = index % UNIQUE_PIECES;

        let side = match side {
            0 => Side::White,
            1 => Side::Black,
            _ => panic!("Invalid Side"),
        };

        let kind = match kind {
            0 => Kind::King,
            1 => Kind::Queen,
            2 => Kind::Rook,
            3 => Kind::Bishop,
            4 => Kind::Knight,
            5 => Kind::Pawn,
            _ => panic!("Invalid Kind"),
        };
        Piece::new(kind, side)
    }
}
