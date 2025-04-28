use super::{moves::*, zobrist::*};
use crate::defs::*;

#[derive(Copy, Clone)]
pub struct GameState {
    pub active_color: u8,
    pub castling: u8,
    pub halfmove_clock: u16,
    pub en_passant: Option<u8>,
    pub fullmove_number: u16,
    pub zobrist_key: u64,
    pub game_phase: i16,                    // opening/midgame/endgame
    pub material: [u16; NrOf::PIECE_TYPES], // piece values
    pub psqt: [i16; NrOf::PIECE_TYPES],     // square values per piece
    pub next_move: Move,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            active_color: 0,
            castling: 0,
            halfmove_clock: 0,
            en_passant: None,
            zobrist_key: 0, // TODO: replace with actual u64 from generated from ZobristRandoms
            fullmove_number: 0,
            // TODO: no clue what material stores
            material: [0; NrOf::PIECE_TYPES],
            psqt: [0; NrOf::PIECE_TYPES],
            game_phase: 0,
            next_move: Move::default(),
        }
    }
    pub fn clear(&mut self) {
        self.active_color = 0;
        self.castling = 0;
        self.halfmove_clock = 0;
        self.en_passant = None;
        self.zobrist_key = 0;
        self.fullmove_number = 0;
        self.material = [0u16; NrOf::PIECE_TYPES];
        self.psqt = [0; NrOf::PIECE_TYPES];
        self.game_phase = 0;
        self.next_move = Move::default();
    }
}
