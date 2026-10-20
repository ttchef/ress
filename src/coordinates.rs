#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rank(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct File(u8);

/// Defined by LERF Mapping (Little endian rank file)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    rank: Rank,
    file: File,
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

// TODO: Maybe a square like enum with a1 a2 a3 etc...
