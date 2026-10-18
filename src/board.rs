use crate::error::RessErr;
use crate::piece::{Kind, Piece, Side};

pub const SIZE: usize = 8;

#[derive(Copy, Clone, PartialEq)]
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

impl Into<Bitboard> for Square {
    fn into(self) -> Bitboard {
        Bitboard(1 << self.index())
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn set(&mut self, square: Square) {
        self.0 |= 1 << square.index();
    }

    pub fn unset(&mut self, square: Square) {
        self.0 &= !(1 << square.index());
    }

    pub fn mask(f: impl Fn(usize, usize) -> bool) -> Bitboard {
        let mut bitboard = Bitboard(0);

        for row in 0..SIZE {
            for col in 0..SIZE {
                if f(row, col) {
                    bitboard.set(Square::new(row, col).unwrap());
                }
            }
        }
        bitboard
    }

    pub fn get_row(&self) -> Option<usize> {
        if self.0.count_ones() == 1 {
            for row in 0..SIZE {
                let mask = Bitboard::mask(|r, _| r == row);
                if self.0 & mask.0 > 0 {
                    return Some(row);
                }
            }
        }
        None
    }

    pub fn get_col(&self) -> Option<usize> {
        if self.0.count_ones() == 1 {
            for col in 0..SIZE {
                let mask = Bitboard::mask(|_, c| c == col);
                if self.0 & mask.0 > 0 {
                    return Some(col);
                }
            }
        }
        None
    }

    pub fn get_pos(&self) -> Option<(usize, usize)> {
        if let Some(row) = self.get_row()
            && let Some(col) = self.get_col()
        {
            return Some((row, col));
        }
        None
    }

    fn shift(&self, amount: i32) -> Self {
        match amount.signum() {
            -1 => Bitboard(self.0 >> amount.abs()),
            1 => Bitboard(self.0 << amount),
            0 => Bitboard(self.0),
            _ => panic!("What the hell happend that we got here???"),
        }
    }

    // Not use of square because a offset coud take in smth like -1
    // whish a square is not capable of
    fn offset(&self, row: i32, col: i32) -> Self {
        if let Some(unit_col) = self.get_col() {
            if col > 0 {
                if unit_col as i32 + col >= SIZE as i32 {
                    return Bitboard(0);
                }
            } else if col < 0 {
                if unit_col as i32 + col <= 0 as i32 {
                    return Bitboard(0);
                }
            }
        }
        self.shift(row * SIZE as i32 + col)
    }

    // Returns true if bit on this square is set
    pub fn get(&self, square: Square) -> bool {
        self.0 & (1 << square.index()) != 0
    }
}

impl std::ops::Shl<usize> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
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

    fn unset(&mut self, square: Square) {
        if let Some(piece) = self.get(square) {
            self.team[Side::White as usize].unset(square);
            self.team[Side::Black as usize].unset(square);
            self.pieces[piece.kind as usize].unset(square);
        }
    }

    pub fn clicked(&mut self, square: Square) {
        if let Some(active) = self.active
            && let Some(piece) = self.get(active)
        {
            let moves = self.get_moves(active, piece);
            if moves & square.into() > 0.into() {
                self.active = None;
                self.set(square, piece);
                self.unset(active);

                return;
            }
        }

        if let Some(active) = self.active
            && active == square
        {
            self.active = None;
        } else {
            self.active = Some(square);
        }
    }

    pub fn active_square(&self) -> Option<Square> {
        self.active
    }

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

    pub fn get_moves(&self, square: Square, piece: Piece) -> Bitboard {
        let unit = Bitboard(1 << square.index());
        let team_mask = !self.team[piece.side as usize];

        match piece.kind {
            Kind::King => {
                let possible = unit.offset(1, 0)
                    | unit.offset(-1, 0)
                    | unit.offset(0, 1)
                    | unit.offset(0, -1)
                    | unit.offset(1, 1)
                    | unit.offset(1, -1)
                    | unit.offset(-1, 1)
                    | unit.offset(-1, -1);
                possible & team_mask
            }
            Kind::Pawn => {
                let (side, rank) = match piece.side {
                    Side::White => (1, 4),
                    Side::Black => (-1, 3),
                };

                let mut possible = unit.offset(-1 * side, 0);
                let double_move = unit.offset(-2 * side, 0);

                let rank_mask = Bitboard(0xff << rank * SIZE);
                possible |= double_move & rank_mask;

                possible & team_mask
            }
            Kind::Rook => {
                let (row, col) = unit.get_pos().unwrap();

                let mut possible = Bitboard(0);

                for row in (row + 1)..SIZE {
                    let pos = Square::new(row, col).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }
                for row in (0..row).rev() {
                    let pos = Square::new(row, col).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }
                for col in (col + 1)..SIZE {
                    let pos = Square::new(row, col).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }
                for col in (0..col).rev() {
                    let pos = Square::new(row, col).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                possible
            }
            Kind::Bishop => {
                let (row, col) = unit.get_pos().unwrap();

                let mut possible = Bitboard(0);

                let br_dist = SIZE - row.max(col);
                for i in 1..br_dist {
                    let pos = Square::new(row + i, col + i).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                let tl_dist = row.min(col);
                for i in 1..=tl_dist {
                    let pos = Square::new(row - i, col - i).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                let tr_dist = row.min(SIZE - col);
                for i in 1..tr_dist {
                    let pos = Square::new(row - i, col + i).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                let bl_dist = col.min(SIZE - row);
                for i in 1..bl_dist {
                    let pos = Square::new(row + i, col - i).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                possible
            }
            Kind::Queen => {
                let (row, col) = unit.get_pos().unwrap();

                let mut possible = Bitboard(0);

                for row in (row + 1)..SIZE {
                    let pos = Square::new(row, col).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }
                for row in (0..row).rev() {
                    let pos = Square::new(row, col).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }
                for col in (col + 1)..SIZE {
                    let pos = Square::new(row, col).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }
                for col in (0..col).rev() {
                    let pos = Square::new(row, col).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                let br_dist = SIZE - row.max(col);
                for i in 1..br_dist {
                    let pos = Square::new(row + i, col + i).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                let tl_dist = row.min(col);
                for i in 1..=tl_dist {
                    let pos = Square::new(row - i, col - i).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                let tr_dist = row.min(SIZE - col);
                for i in 1..tr_dist {
                    let pos = Square::new(row - i, col + i).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                let bl_dist = col.min(SIZE - row);
                for i in 1..bl_dist {
                    let pos = Square::new(row + i, col - i).unwrap();
                    if let Some(other) = self.get(pos) {
                        if piece.side != other.side {
                            possible.set(pos);
                        }
                        break;
                    }
                    possible.set(pos);
                }

                possible
            }
            Kind::Knight => {
                let possible = unit.offset(2, 1)
                    | unit.offset(2, -1)
                    | unit.offset(-2, 1)
                    | unit.offset(-2, -1)
                    | unit.offset(1, 2)
                    | unit.offset(1, -2)
                    | unit.offset(-1, 2)
                    | unit.offset(-1, -2);
                possible & team_mask
            }
        }
    }
}
