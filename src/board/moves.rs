use crate::{defs::*, pieces::Pieces};

#[derive(Clone, Copy)]
pub struct Move {
    pub piece: Pieces,
    pub from: u8,
    pub to: u8,
    pub kind: MoveType,
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
}

#[derive(Clone, Copy)]
pub enum MoveType {
    Regular,
    Capture(Pieces),
    Promotion(Pieces),
    Castle(Castle),
}

#[derive(Clone, Copy)]
pub enum Castle {
    Kingside,
    Queenside,
}
