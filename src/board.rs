use crate::error::RessErr;
use crate::piece::{Kind, Piece, Side};

pub const SIZE: usize = 8;

#[derive(Copy, Clone)]
pub struct Square {
    row: usize,
    col: usize,
}

impl Square {
    pub const fn new(row: usize, col: usize) -> Result<Self, RessErr> {
        if row >= SIZE || col >= SIZE {
            return Err(RessErr::CoordOutOfRange);
        }
        Ok(Self { row, col })
    }

    fn index(&self) -> usize {
        self.row * SIZE + self.col
    }
}

#[derive(Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn set(&mut self, square: Square) {
        self.0 |= 1 << square.index()
    }

    // Returns true if bit on this square is set
    pub fn get(&self, square: Square) -> bool {
        self.0 & (1 << square.index()) != 0
    }
}

// [0][0] is black back rank
pub struct Board {
    pieces: [Bitboard; 6],
    team: [Bitboard; 2],
    active: Option<Square>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [Bitboard(0); 6],
            team: [Bitboard(0); 2],
            active: None,
        }
    }

    pub fn standard() -> Self {
        let mut board = Board::new();

        let back_rank = [
            Kind::Rook,
            Kind::Knight,
            Kind::Bishop,
            Kind::Queen,
            Kind::King,
            Kind::Bishop,
            Kind::Knight,
            Kind::Rook,
        ];

        for col in 0..SIZE {
            board.set(
                Square::new(0, col).unwrap(),
                Piece::new(back_rank[col], Side::Black),
            );
            board.set(
                Square::new(1, col).unwrap(),
                Piece::new(Kind::Pawn, Side::Black),
            );
            board.set(
                Square::new(6, col).unwrap(),
                Piece::new(Kind::Pawn, Side::White),
            );
            board.set(
                Square::new(7, col).unwrap(),
                Piece::new(back_rank[col], Side::White),
            );
        }

        board
    }

    fn set(&mut self, square: Square, piece: Piece) {
        self.team[piece.side as usize].set(square);
        self.pieces[piece.kind as usize].set(square);
    }

    pub fn clicked(&mut self, square: Square) {}

    pub fn get(&self, square: Square) -> Option<Piece> {
        let mut team_index = None;
        for (i, team) in self.team.iter().enumerate() {
            if team.get(square) {
                team_index = Some(i);
                break;
            }
        }
        let Some(team_index) = team_index else {
            return None;
        };
        let team = match team_index {
            0 => Side::White,
            1 => Side::Black,
            other => panic!("Invalid team index: {other}"),
        };

        let mut piece_index = None;
        for (i, piece_type) in self.pieces.iter().enumerate() {
            if piece_type.get(square) {
                piece_index = Some(i);
                break;
            }
        }

        let mut piece = None;
        if let Some(piece_index) = piece_index {
            match piece_index {
                0 => piece = Some(Piece::new(Kind::King, team)),
                1 => piece = Some(Piece::new(Kind::Queen, team)),
                2 => piece = Some(Piece::new(Kind::Rook, team)),
                3 => piece = Some(Piece::new(Kind::Bishop, team)),
                4 => piece = Some(Piece::new(Kind::Knight, team)),
                5 => piece = Some(Piece::new(Kind::Pawn, team)),
                other => panic!("Inalid piece id: {other}"),
            }
        }
        piece
    }
}

#[test]
fn bitboard_square() {
    let board = Bitboard(1 << (2 * SIZE + 4));
    assert!(board.get(Square::new(2, 4).unwrap()));
}
