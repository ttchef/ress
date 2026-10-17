const SIZE: usize = 8;

enum Side {
    White,
    Black
}

#[derive(Clone, Copy, Debug)]
struct PieceBitBoard {
    white: u64,
    black: u64,
}

impl PieceBitBoard {
    fn set(&mut self) {}
    fn get_mut(&mut self, side: Side) -> &mut u64 {
        match side {
            Side::White => &mut self.white,
            Side::Black => &mut self.black,
        }
    }
}

pub struct Board {
    king: PieceBitBoard,
    queen: PieceBitBoard,
    rook: PieceBitBoard,
    bishop: PieceBitBoard,
    knight: PieceBitBoard,
    pawn: PieceBitBoard,
}

impl Board {
    fn standard() -> Self {
        let king = PieceBitBoard
        Self {}
    }
}
