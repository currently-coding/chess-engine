use std::ops::RangeInclusive;
// TYPE ALIASES
pub type Bitboard = u64;
pub type Square = u8;
pub const EMPTY: u64 = 0;

// MAX MOVES
pub const MAX_GAME_MOVES: usize = 1024;
pub const MAX_MOVE_RULE: usize = 100;

// FEN STRINGS
pub const SHORT_FEN_LENGTH: usize = 4;
pub const FEN_LENGTH: usize = 6;
pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const EN_PASSANT_SQUARES_WHITE: RangeInclusive<Square> = Squares::A3..=Squares::H3;
pub const EN_PASSANT_SQUARES_BLACK: RangeInclusive<Square> = Squares::A6..=Squares::H6;
pub const EN_PASSANT_START_SQUARES_WHITE: RangeInclusive<Square> = Squares::A2..=Squares::H2;
pub const EN_PASSANT_START_SQUARES_BLACK: RangeInclusive<Square> = Squares::A7..=Squares::H7;
pub const EN_PASSANT_END_SQUARES_WHITE: RangeInclusive<Square> = Squares::A4..=Squares::H4;
pub const EN_PASSANT_END_SQUARES_BLACK: RangeInclusive<Square> = Squares::A5..=Squares::H5;
// ---
pub struct Castling;
impl Castling {
    pub const WK: u8 = 1; // 0001
    pub const WQ: u8 = 2; // 0010
    pub const BK: u8 = 4; // 0100
    pub const BQ: u8 = 8; // 1000
    pub const ALL: u8 = 15; //1111
}

// BOARD NUMBER CONSTANTS
pub struct NrOf;
impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_PERMISSIONS: usize = 16; // 0-15
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const SIDES: usize = 2;
}

pub const WHITE: u8 = 0;
pub const BLACK: u8 = 1;

// TRANSLATION
#[rustfmt::skip]
pub const SQUARE_NAME: [&str; NrOf::SQUARES] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
];
pub const PIECE_NAME: [&str; NrOf::PIECE_TYPES + 1] =
    ["King", "Queen", "Rook", "Bishop", "Knight", "Pawn", "-"];
pub const PIECE_CHAR_CAPS: [&str; NrOf::PIECE_TYPES + 1] = ["K", "Q", "R", "B", "N", "P", "_"];
pub const PIECE_CHAR_SMALL: [&str; NrOf::PIECE_TYPES + 1] = ["k", "q", "r", "b", "n", "", ""];

// IMPORTANT BOARD SQUARES
pub struct Squares;
impl Squares {
    // White side squares that are important for castling
    pub const A1: Square = 0;
    pub const B1: Square = 1;
    pub const C1: Square = 2;
    pub const D1: Square = 3;
    pub const E1: Square = 4;
    pub const F1: Square = 5;
    pub const G1: Square = 6;
    pub const H1: Square = 7;

    // Black side squares that are important for castling
    pub const A8: Square = 56;
    pub const B8: Square = 57;
    pub const C8: Square = 58;
    pub const D8: Square = 59;
    pub const E8: Square = 60;
    pub const F8: Square = 61;
    pub const G8: Square = 62;
    pub const H8: Square = 63;

    // all En-Passant related squares/ranges
    // White EP-squares start/end
    pub const A3: Square = 16;
    pub const H3: Square = 23;

    // Black EP-squares start/end
    pub const A6: Square = 40;
    pub const H6: Square = 47;

    // White EP-Squares start
    pub const A2: Square = 8;
    pub const H2: Square = 15;

    // Black EP-Squares start
    pub const A7: Square = 48;
    pub const H7: Square = 55;

    // White EP-Squares end
    pub const A4: Square = 24;
    pub const H4: Square = 31;

    // Black EP-Squares end
    pub const A5: Square = 32;
    pub const H5: Square = 39;
}

// BOARD RANKS AND FILES
pub struct Files;
impl Files {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const G: usize = 6;
    pub const H: usize = 7;
}

pub struct Ranks;
impl Ranks {
    pub const R1: usize = 0;
    pub const R2: usize = 1;
    pub const R4: usize = 3;
    pub const R5: usize = 4;
    pub const R7: usize = 6;
    pub const R8: usize = 7;
}

// BOARD RANGES
pub struct RangeOf;
impl RangeOf {
    pub const RANKS: RangeInclusive<u8> = (Ranks::R1 as u8)..=(Ranks::R8 as u8);
    pub const FILES: RangeInclusive<u8> = (Files::A as u8)..=(Files::H as u8);
    pub const SQUARES: RangeInclusive<Square> = 0..=63;
}
