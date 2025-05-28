use core::fmt;

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Pieces {
    Pawn = 0,
    Bishop = 1,
    Knight = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
    Empty = 6,
}

impl TryFrom<usize> for Pieces {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Pieces::Pawn),
            1 => Ok(Pieces::Bishop),
            2 => Ok(Pieces::Knight),
            3 => Ok(Pieces::Rook),
            4 => Ok(Pieces::Queen),
            5 => Ok(Pieces::King),
            6 => Ok(Pieces::Empty),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Pieces {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Pieces::try_from(value as usize) // reuse logic
    }
}
impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Pieces::Pawn => "P",
            Pieces::Bishop => "B",
            Pieces::Knight => "N",
            Pieces::Rook => "R",
            Pieces::Queen => "Q",
            Pieces::King => "K",
            Pieces::Empty => ".",
        };
        write!(f, "{}", symbol)
    }
}
