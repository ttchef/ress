#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn empty() -> Self {
        Bitboard(0)
    }
}
