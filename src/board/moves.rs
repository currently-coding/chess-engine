use core::fmt;

use crate::{pieces::Pieces, Square, Squares};

#[derive(Clone, Copy, PartialEq)]
pub struct Move {
    piece: Pieces,
    from: u8,
    to: u8,
    kind: MoveType,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveType {
    Regular,
    Capture(Pieces),
    Promotion(Pieces),
    Castle(Castle),
}

// impl PartialEq for MoveType {
//     fn eq(&self, other: &MoveType) -> bool {
//         match (self, other) {
//             (MoveType::Regular, MoveType::Regular) => true, // compare Regular
//             (MoveType::Capture(a), MoveType::Capture(b)) => a == b, // compare Capture inner
//             (MoveType::Promotion(a), MoveType::Promotion(b)) => a == b, // compare Promotion inner
//             (MoveType::Castle(a), MoveType::Castle(b)) => a == b, // compare Castle inner
//             _ => false,                                     // different variants
//         }
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Castle {
    Kingside,
    Queenside,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Move {{ piece: {:?} from {} to {}. Kind: {:?}}}",
            self.piece, self.from, self.to, self.kind
        )
    }
}

// TODO: Optimizations:
// 1. use a single u64 to store all data by bitshifting etc. -> create functions to retrieve data
