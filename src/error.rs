#[derive(Debug)]
pub enum RessErr {
    CoordOutOfRange,
}

impl std::fmt::Display for RessErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoordOutOfRange => write!(f, "coordinates out of valid range"),
        }
    }
}

impl std::error::Error for RessErr {}
