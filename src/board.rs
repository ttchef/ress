use crate::piece::{Kind, Piece, Side};

pub const SIZE: usize = 8;

#[derive(Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn set(&mut self, row: usize, col: usize) {
        self.0 |= 1 << (row * SIZE + col);
    }

    // Returns true if bit on this square is set
    pub fn square(&self, row: usize, col: usize) -> bool {
        self.0 & (1 << (row * SIZE + col)) != 0
    }
}

// [0][0] is black back rank
pub struct Board {
    pieces: [Bitboard; 6],
    team: [Bitboard; 2],
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [Bitboard(0); 6],
            team: [Bitboard(0); 2],
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
            board.set(0, col, Piece::new(back_rank[col], Side::Black));
            board.set(1, col, Piece::new(Kind::Pawn, Side::Black));
            board.set(6, col, Piece::new(Kind::Pawn, Side::White));
            board.set(7, col, Piece::new(back_rank[col], Side::White));
        }
        board.set(7, 0, Piece::new(Kind::King, Side::Black));

        board
    }

    fn set(&mut self, row: usize, col: usize, piece: Piece) {
        self.team[piece.side as usize].set(row, col);
        self.pieces[piece.kind as usize].set(row, col);
    }

    pub fn get_square(&self, row: usize, col: usize) -> Option<Piece> {
        let mut team_index = None;
        for (i, team) in self.team.iter().enumerate() {
            if team.square(row, col) {
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
            if piece_type.square(row, col) {
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
    assert!(board.square(2, 4));
}
