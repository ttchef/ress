const UNIQUE_PIECES: usize = 6;

pub struct MoveVector {
    pub row: i32,
    pub col: i32,
}

impl MoveVector {
    const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

const KNIGHT_MOVE_VECTOR: [MoveVector; 8] = [
    MoveVector::new(2, -1),
    MoveVector::new(2, 1),
    MoveVector::new(1, 2),
    MoveVector::new(1, -2),
    MoveVector::new(-2, -1),
    MoveVector::new(-2, 1),
    MoveVector::new(-1, 2),
    MoveVector::new(-1, -2),
];

#[derive(Copy, Clone, Debug)]
pub enum Side {
    White,
    Black,
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

    pub fn asset_path(&self) -> String {
        let side = match self.side {
            Side::White => "w",
            Side::Black => "b",
        };
        let kind = match self.kind {
            Kind::King => "k",
            Kind::Queen => "q",
            Kind::Rook => "r",
            Kind::Bishop => "b",
            Kind::Knight => "n",
            Kind::Pawn => "p",
        };
        format!("assets/textures/pieces/{}{}.png", side, kind)
    }

    pub fn get_move_vectors(&self) -> &[MoveVector] {
        match self.kind {
            _ => &KNIGHT_MOVE_VECTOR,
        }
    }
}
