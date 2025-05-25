use std::sync::Arc;

use super::{moves::*, zobrist::*};
use crate::defs::*;

#[derive(Copy, Clone, Debug)]
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

impl PartialEq for GameState {
    fn eq(&self, other: &GameState) -> bool {
        println!("COMPARISON:");
        self.debug();
        other.debug();
        self.active_color == other.active_color
            && self.castling == other.castling
            && self.halfmove_clock == other.halfmove_clock
            && self.en_passant == other.en_passant
            && self.fullmove_number == other.fullmove_number
            && self.zobrist_key == other.zobrist_key
            && self.game_phase == other.game_phase
            && self.material == other.material
            && self.psqt == other.psqt
            && self.next_move == other.next_move
    }
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

    pub(crate) fn debug(&self) {
        println!("Game State Debug Information:");
        println!("--------------------------------");
        println!("Active Color: {}", self.active_color);
        println!("Castling: {}", self.castling);
        println!("Halfmove Clock: {}", self.halfmove_clock);
        println!("En Passant: {:?}", self.en_passant);
        println!("Fullmove Number: {}", self.fullmove_number);
        println!("Zobrist Key: {}", self.zobrist_key);
        println!("Game Phase: {}", self.game_phase);
        println!("Material: {:?}", self.material);
        println!("PSQT: {:?}", self.psqt);
        println!("Next Move: {:?}", self.next_move);
        println!("--------------------------------");
    }
}
