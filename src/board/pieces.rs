use core::fmt;

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum Pieces {
    Pawn = 0,
    Bishop = 1,
    Knight = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
    Empty = 6,
}
impl Pieces {
    pub fn from_index(index: usize) -> Pieces {
        match index {
            0 => Pieces::Pawn,
            1 => Pieces::Bishop,
            2 => Pieces::Knight,
            3 => Pieces::Rook,
            4 => Pieces::Queen,
            5 => Pieces::King,
            6 => Pieces::Empty,
            _ => panic!("Pieces index out of range"), // TODO: proper error handling
        }
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
