pub const SIZE: u8 = 8;

/// Defined by LERF Mapping (Little endian rank file)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    pub rank: u8,
    pub file: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compass {
    North = 8,
    NorthEast = 9,
    East = 1,
    SouthEast = -7,
    South = -8,
    SouthWest = -9,
    West = -1,
    NorthWest = 7,
}

impl Square {
    pub const fn new(rank: u8, file: u8) -> Self {
        assert!(rank < SIZE);
        assert!(file < SIZE);
        Self { rank, file }
    }
}
