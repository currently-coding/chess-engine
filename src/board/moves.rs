use core::fmt;

use crate::{pieces::Pieces, Square, Squares};

#[derive(Clone, Copy, PartialEq)]
pub struct Move {
    piece: Pieces,
    from: u8,
    to: u8,
    kind: MoveType,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveType {
    Regular,
    Capture(Pieces),
    Promotion(Pieces),
    Castle(Castle),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Castle {
    Kingside,
    Queenside,
}

impl Move {
    pub fn default() -> Self {
        Self {
            piece: Pieces::Empty,
            from: 0,
            to: 0,
            kind: MoveType::Regular,
        }
    }
    pub fn new(piece: Pieces, from: u8, to: u8, kind: MoveType) -> Self {
        Self {
            piece,
            from,
            to,
            kind,
        }
    }

    pub fn piece(&self) -> Pieces {
        self.piece
    }

    pub fn from(&self) -> u8 {
        self.from
    }

    pub fn to(&self) -> u8 {
        self.to
    }

    pub fn kind(&self) -> MoveType {
        self.kind
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Move{{ {:?} with {:?}: {} -> {} }}",
            self.kind, self.piece, self.from, self.to
        )
    }
}

// TODO: Optimizations:
// 1. use a single u64 to store all data by bitshifting etc. -> create functions to retrieve data
