mod fen;
mod gamestate;
mod history;
mod moves;
pub mod pieces;
mod playmove;
pub mod sides;
mod zobrist;

use std::sync::Arc;

use zobrist::ZobristKey;
use zobrist::ZobristRandoms;

use self::{fen::*, gamestate::GameState, history::GameHistory, pieces::Pieces, sides::Sides};
use crate::defs::*;
use crate::helper;
use crate::helper::*;

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; NrOf::PIECE_TYPES]; Sides::Both as usize],
    pub side: [Bitboard; Sides::Both as usize],
    pub game_state: GameState,
    pub history: GameHistory,
    pub piece_list: [Pieces; NrOf::SQUARES],
    zobrist_randoms: Arc<ZobristRandoms>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [[EMPTY; NrOf::PIECE_TYPES]; Sides::Both as usize],
            side: [EMPTY; Sides::Both as usize],
            game_state: GameState::new(),
            history: GameHistory::new(),
            piece_list: [Pieces::Empty; NrOf::SQUARES],
            zobrist_randoms: Arc::new(ZobristRandoms::new()),
        }
    }
    fn init(&mut self) {
        println!("initializing board...");
        self.init_side_bb();
        self.init_piece_list();
        self.game_state.zobrist_key = self.init_zobrist_key();
        // TODO: init psqt here as well
    }
    pub fn reset(&mut self) {
        self.pieces = [[EMPTY; NrOf::PIECE_TYPES]; Sides::Both as usize];
        self.side = [EMPTY; Sides::Both as usize];
        self.game_state.clear();
        self.history.clear();
        self.piece_list = [Pieces::Empty; NrOf::SQUARES];
    }
    pub fn display(&self) {
        for (count, piece) in self.piece_list.iter().rev().enumerate() {
            if (NrOf::SQUARES - count) % 8 == 0 && count != 0 {
                println!("|");
            }
            print!("|");
            // TODO: change to use PIECE_CHAR_CAPS/SMALL
            let piece_string = match piece {
                Pieces::Empty => PIECE_CHAR_CAPS[6],
                Pieces::Pawn => PIECE_CHAR_CAPS[5],
                Pieces::Knight => PIECE_CHAR_CAPS[4],
                Pieces::Bishop => PIECE_CHAR_CAPS[3],
                Pieces::Rook => PIECE_CHAR_CAPS[2],
                Pieces::Queen => PIECE_CHAR_CAPS[1],
                Pieces::King => PIECE_CHAR_CAPS[0],
            };
            print!("{}", piece_string);
        }
        println!("|");
    }
}

impl Board {
    // TODO:
    // find out where to initialise the zobrist key in game_state
    // change the initializing functions for the board
    // fen should modify an existing board, not create a new one

    pub fn init_zobrist_key(&self) -> ZobristKey {
        println!("initializing ZobristKey");
        let mut key: u64 = 0;
        key ^= self.zobrist_randoms.castling(self.game_state.castling);
        key ^= self.zobrist_randoms.en_passant(self.game_state.en_passant);
        key ^= self
            .zobrist_randoms
            .sides(Sides::from(self.game_state.active_color));

        let mut square: u8;
        let mut tmp_square: Option<u8>;
        // TODO: could be reduced to manually get black and wihte instead of iterating over sides
        for (s_idx, side) in self.pieces.iter().enumerate() {
            for (p_idx, piece) in side.iter().enumerate() {
                let mut p = *piece;
                tmp_square = helper::next_bit(&mut p);
                match tmp_square {
                    Some(s) => square = s,
                    None => continue,
                }
                key ^= self.zobrist_randoms.pieces(
                    Sides::from(s_idx as u8),
                    Pieces::from_index(p_idx),
                    square,
                );
            }
        }
        println!("Done initializing ZobristKey");
        key
    }

    pub fn fen(fen_input: Option<&str>) -> Self {
        let mut board = Board::new();
        let mut fen = FEN_START_POSITION;
        if let Some(string) = fen_input {
            fen = string;
        }
        let result = board.fen_setup(fen);
        if let Err(e) = result {
            println!("Error: {}", e);
        }
        board
    }
    pub fn we(&self) -> usize {
        self.game_state.active_color as usize
    }
    pub fn occupancy(&self) -> Bitboard {
        self.side[Sides::White as usize] | self.side[Sides::Black as usize]
    }
    pub fn opponent(&self) -> usize {
        (self.game_state.active_color ^ 1) as usize
    }
    pub fn get_pieces(&self, side: Sides, piece: Pieces) -> u64 {
        self.pieces[side as usize][piece as usize]
    }
    pub fn get_side(&self, side: Sides) -> Bitboard {
        self.side[side as usize]
    }
    pub fn king(&self, side: Sides) -> Square {
        self.get_pieces(side, Pieces::King).trailing_zeros() as Square
    }

    // seems to work
    fn init_piece_list(&mut self) {
        let bb_w = &self.pieces[Sides::White as usize]; // White piece bitboards
        let bb_b = &self.pieces[Sides::Black as usize]; // Black piece bitboards

        let mut piece_list = [Pieces::Empty; NrOf::SQUARES];

        (0..NrOf::SQUARES).for_each(|square| {
            let mask = helper::get_bitmask(square as u8);

            bb_b.iter().enumerate().for_each(|(piece_num, &bb)| {
                if bb & mask != 0 {
                    piece_list[square] = Pieces::from_index(piece_num);
                }
            });

            bb_w.iter().enumerate().for_each(|(piece_num, &bb)| {
                if bb & mask != 0 {
                    piece_list[square] = Pieces::from_index(piece_num);
                }
            });
        });

        self.piece_list = piece_list;
    }

    // pretty sure that works (although no tests yet)
    fn init_side_bb(&mut self) {
        for (idx, side) in self.pieces.iter().enumerate() {
            for bb in side {
                self.side[idx] |= bb;
            }
        }
    }
    pub fn fen_setup(&mut self, fen_string: &str) -> Result<(), FenError> {
        println!("Setting up from FEN: {}", fen_string);
        let parts = split_fen_string(fen_string)?;
        // only modify copy in case the fen is invalid
        let mut tmp_board = self.clone();
        tmp_board.reset();
        let parsers = create_fen_parsers();
        for (part, parser) in parts.iter().zip(parsers.iter()) {
            parser(&mut tmp_board, part)?;
        }
        // self.pieces and some stuff is set up. now init self.side, self.piece_list, etc.
        tmp_board.init();
        println!("--------");
        *self = tmp_board;
        self.debug();
        Ok(())
    }
    pub fn debug(&self) {
        println!("Bitboards");
        for side in self.pieces {
            println!("--- ");
            for bitboard in side {
                println!("{:064b}", bitboard);
            }
        }
        println!("Sides");
        for bitboard in self.side {
            println!("{:064b}", bitboard);
        }
        println!("ZobristKey: {}", self.game_state.zobrist_key);
    }
    pub fn is_dark_square(&self, square: Square) -> bool {
        let rank = square / 8;
        let file = square - rank * 8;
        let even_rank = (rank & 1) == 0;
        let even_file = (file & 1) == 0;
        (even_file && even_rank) || (!even_file && !even_rank)
    }
    pub fn draw_by_insufficient_material(self) -> bool {
        let w = self.get_bitboards(Sides::White);
        let b = self.get_bitboards(Sides::Black);

        !(w[Pieces::Queen as usize] > 0
            || w[Pieces::Rook as usize] > 0
            || w[Pieces::Pawn as usize] > 0
            || b[Pieces::Queen as usize] > 0
            || b[Pieces::Rook as usize] > 0
            || b[Pieces::Pawn as usize] > 0
            || self.has_bishop_pair(Sides::Black)
            || self.has_bishop_pair(Sides::White)
            || w[Pieces::Knight as usize].count_ones() >= 3
            || b[Pieces::Knight as usize].count_ones() >= 3
            || (w[Pieces::Knight as usize].count_ones() >= 1 && w[Pieces::Bishop as usize] >= 1)
            || (b[Pieces::Knight as usize].count_ones() >= 1 && b[Pieces::Bishop as usize] >= 1))
    }
    fn has_bishop_pair(&self, side: Sides) -> bool {
        let mut bb = self.get_piece_bb(side, Pieces::Bishop);
        let mut square;
        let mut dark_squared_bishop = false;
        let mut light_squared_bishop = false;
        while bb > 0 {
            square = bb.trailing_zeros();
            bb ^= helper::get_bitmask(square as u8);
            if self.is_dark_square(square as u8) {
                dark_squared_bishop = true;
            } else {
                light_squared_bishop = true;
            }
            if dark_squared_bishop && light_squared_bishop {
                return true;
            }
        }
        false
    }
    fn get_piece_bb(&self, side: Sides, piece: Pieces) -> u64 {
        self.pieces[side as usize][piece as usize]
    }
    pub fn get_bitboards(&self, side: Sides) -> [u64; NrOf::PIECE_TYPES] {
        self.pieces[side as usize]
    }
}
#[cfg(test)]
mod tests {
    use crate::board::Pieces;
    use crate::Board;

    use super::sides::Sides;

    #[test]
    fn test_init_side_bb() {
        let mut board = Board::new();
        board.pieces[Sides::White as usize][Pieces::Pawn as usize] = 0x0FF00000000;
        board.init_side_bb();
        assert_eq!(board.side[Sides::White as usize], 0x0FF00000000);
    }
    #[test]
    fn test_is_dark_square() {
        let board = Board::new();
        assert!(board.is_dark_square(0));
        assert!(!board.is_dark_square(1));
        assert!(!board.is_dark_square(7));
        assert!(board.is_dark_square(2));
        assert!(!board.is_dark_square(8));
        assert!(!board.is_dark_square(10));
        assert!(!board.is_dark_square(19));
        assert!(board.is_dark_square(9));
        assert!(board.is_dark_square(63));
        assert!(board.is_dark_square(0));
    }
    #[test]
    fn test_has_bishop_pair() {
        let mut board = Board::new();
        board.pieces[Sides::White as usize][Pieces::Bishop as usize] = 0b01100000000u64;
        board.pieces[Sides::Black as usize][Pieces::Bishop as usize] = 0b0000000010u64;
        assert!(board.has_bishop_pair(Sides::White));
        assert!(!board.has_bishop_pair(Sides::Black));
        board.pieces[Sides::White as usize][Pieces::Bishop as usize] = 0b000110010010u64;
        board.pieces[Sides::Black as usize][Pieces::Bishop as usize] = 0b000000000u64;
        assert!(board.has_bishop_pair(Sides::White));
        assert!(!board.has_bishop_pair(Sides::Black));
        board.pieces[Sides::White as usize][Pieces::Bishop as usize] = 0b000010000u64;
        board.pieces[Sides::Black as usize][Pieces::Bishop as usize] = 0b000110010010u64;
        assert!(!board.has_bishop_pair(Sides::White));
        assert!(board.has_bishop_pair(Sides::Black));
    }
    #[test]
    fn test_has_sufficent_material() {
        let board = Board::fen(None);
        board.debug();
        assert!(!board.draw_by_insufficient_material());
        let board = Board::fen(Some("8/8/8/5k2/8/7Q/1K6/8 w - - 0 1"));
        assert!(!board.draw_by_insufficient_material());
        let board = Board::fen(Some("8/8/3b4/5k2/8/8/1K6/8 w - - 0 1"));
        assert!(board.draw_by_insufficient_material());
    }
}
