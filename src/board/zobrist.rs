use crate::defs::*;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

use super::pieces::Pieces;

pub type ZobristKey = u64;

#[derive(Debug)]
pub struct ZobristRandoms {
    // TODO: one could reduce the complexity by creating a side random, that will be xor-ed with
    // each rnd_piece when needed instead of creating twice as much keys
    rnd_pieces: [[[u64; NrOf::SQUARES]; NrOf::PIECE_TYPES]; NrOf::SIDES],
    rnd_castling: [u64; NrOf::CASTLING_PERMISSIONS],
    rnd_sides: [u64; NrOf::SIDES],
    // TODO: change to only use 16 en-passant keys
    rnd_en_passant: [u64; NrOf::SQUARES],
}

impl ZobristRandoms {
    pub fn new() -> Self {
        let seed: [u8; 32] = [42; 32];
        let mut rng = ChaChaRng::from_seed(seed);
        let mut zobrist_randoms = Self {
            rnd_pieces: [[[0u64; NrOf::SQUARES]; NrOf::PIECE_TYPES]; NrOf::SIDES],
            rnd_castling: [0u64; NrOf::CASTLING_PERMISSIONS],
            rnd_sides: [0u64; NrOf::SIDES],
            rnd_en_passant: [0u64; NrOf::SQUARES],
        };

        zobrist_randoms.rnd_pieces.iter_mut().for_each(|side| {
            side.iter_mut()
                .for_each(|piece| piece.iter_mut().for_each(|square| *square = rng.next_u64()))
        });

        zobrist_randoms
            .rnd_castling
            .iter_mut()
            .for_each(|permission| *permission = rng.next_u64());

        zobrist_randoms
            .rnd_sides
            .iter_mut()
            .for_each(|side| *side = rng.next_u64());
        zobrist_randoms
            .rnd_en_passant
            .iter_mut()
            .for_each(|en_passant| *en_passant = rng.next_u64());

        zobrist_randoms
    }

    pub fn castling(&self, permission: u8) -> ZobristKey {
        self.rnd_castling[permission as usize]
    }

    pub fn sides(&self, side: u8) -> ZobristKey {
        self.rnd_sides[side as usize]
    }

    pub fn en_passant(&self, square: Option<Square>) -> ZobristKey {
        if let Some(sq) = square {
            self.rnd_en_passant[sq as usize]
        } else {
            0
        }
    }
    pub fn pieces(&self, side: u8, piece: Pieces, square: Square) -> ZobristKey {
        self.rnd_pieces[side as usize][piece as usize][square as usize]
    }
}
