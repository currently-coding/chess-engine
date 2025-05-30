mod fen;
mod gamestate;
mod history;
pub mod moves;
pub mod pieces;
mod playmove;
pub mod sides;
mod zobrist;

use core::fmt;
use std::sync::Arc;

use zobrist::ZobristKey;
use zobrist::ZobristRandoms;

use self::{fen::*, gamestate::GameState, history::GameHistory, pieces::Pieces};
use crate::defs::*;
use crate::helper;
use crate::helper::*;

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; NrOf::PIECE_TYPES]; NrOf::SIDES],
    pub side: [Bitboard; NrOf::SIDES],
    pub game_state: GameState,
    pub history: GameHistory,
    pub piece_list: [Pieces; NrOf::SQUARES],
    zobrist_randoms: Arc<ZobristRandoms>,
    // TODO: make history start with first real gamestate instead of finst one being empty
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        println!("COMPARISON:");
        self.game_state.debug();
        other.game_state.debug();

        self.pieces == other.pieces && // compare pieces
        self.side == other.side && // compare side
        self.game_state == other.game_state && // compare game state
        self.piece_list == other.piece_list // ignore zobrist_randoms
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Board")
            .field("pieces", &self.pieces)
            .field("side", &self.side)
            .field("game_state", &self.game_state)
            .field("history", &&self.history.get_current())
            .finish()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [[EMPTY; NrOf::PIECE_TYPES]; NrOf::SIDES],
            side: [EMPTY; NrOf::SIDES],
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
        self.pieces = [[EMPTY; NrOf::PIECE_TYPES]; NrOf::SIDES];
        self.side = [EMPTY; NrOf::SIDES];
        self.game_state.clear();
        self.history.clear();
        self.piece_list = [Pieces::Empty; NrOf::SQUARES];
    }
    pub fn display(&self) {
        println!("Board:");
        // start at square 56 = top left
        let mut file = 0;
        let mut rank = 7;
        let mut square;
        print!("|");
        loop {
            square = (rank * 8) + file;
            print!("{}|", self.piece_list[square]);
            if file == 7 {
                file = 0;
                if rank == 0 {
                    break;
                }
                rank -= 1;

                println!();
                print!("|")
            } else {
                file += 1;
            }
        }
        println!();
    }
}

impl Board {
    pub fn init_zobrist_key(&self) -> ZobristKey {
        println!("initializing ZobristKey");
        let mut key: u64 = 0;
        key ^= self.zobrist_randoms.castling(self.game_state.castling);
        key ^= self.zobrist_randoms.en_passant(self.game_state.en_passant);
        key ^= self.zobrist_randoms.sides(self.game_state.active_color);

        let mut square: u8;
        let mut tmp_square: Option<u8>;
        // TODO: could be reduced to manually get black and wihte instead of iterating over sides
        for (side, side_bb) in self.pieces.iter().enumerate() {
            for (piece, piece_bb) in side_bb.iter().enumerate() {
                let mut p = *piece_bb;
                tmp_square = helper::next_bit(&mut p);
                match tmp_square {
                    Some(s) => square = s,
                    None => continue,
                }
                key ^= self.zobrist_randoms.pieces(
                    side as u8,
                    Pieces::try_from(piece).unwrap(),
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
    pub fn we(&self) -> u8 {
        self.game_state.active_color
    }
    pub fn occupancy(&self) -> Bitboard {
        self.side[WHITE as usize] | self.side[BLACK as usize]
    }
    pub fn opponent(&self) -> u8 {
        self.game_state.active_color ^ 1
    }
    pub fn get_pieces(&self, side: u8, piece: Pieces) -> u64 {
        self.pieces[side as usize][piece as usize]
    }
    pub fn get_side(&self, side: u8) -> Bitboard {
        self.side[side as usize]
    }
    pub fn king(&self, side: u8) -> Square {
        self.get_pieces(side, Pieces::King).trailing_zeros() as Square
    }

    // seems to work
    fn init_piece_list(&mut self) {
        let bb_w = &self.pieces[WHITE as usize]; // White piece bitboards
        let bb_b = &self.pieces[BLACK as usize]; // Black piece bitboards

        let mut piece_list = [Pieces::Empty; NrOf::SQUARES];

        (0..NrOf::SQUARES).for_each(|square| {
            let mask = helper::get_bitmask(square as u8);

            bb_b.iter().enumerate().for_each(|(piece_num, &bb)| {
                if bb & mask != 0 {
                    piece_list[square] = Pieces::try_from(piece_num).unwrap();
                }
            });

            bb_w.iter().enumerate().for_each(|(piece_num, &bb)| {
                if bb & mask != 0 {
                    if let Ok(piece) = Pieces::try_from(piece_num) {
                        piece_list[square] = piece;
                    } else {
                        panic!("Trying to convert unsupported type to `Piece`");
                    }
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
        tmp_board.debug_bb();
        tmp_board.init();
        println!("--------");
        *self = tmp_board;
        self.debug_bb();
        Ok(())
    }

    pub fn debug_piece_list(&self) {
        println!("Piecelist");
        for piece in self.piece_list {
            print!("{}, ", piece);
        }
    }
    pub fn debug_bb(&self) {
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
    }
    pub fn debug_all(&self) {
        self.debug_bb();
        self.game_state.debug();
    }
    pub fn is_dark_square(&self, square: Square) -> bool {
        let rank = square / 8;
        let file = square - rank * 8;
        let even_rank = (rank & 1) == 0;
        let even_file = (file & 1) == 0;
        (even_file && even_rank) || (!even_file && !even_rank)
    }
    pub fn draw_by_insufficient_material(self) -> bool {
        let w = self.get_bitboards(WHITE);
        let b = self.get_bitboards(BLACK);

        !(w[Pieces::Queen as usize] > 0
            || w[Pieces::Rook as usize] > 0
            || w[Pieces::Pawn as usize] > 0
            || b[Pieces::Queen as usize] > 0
            || b[Pieces::Rook as usize] > 0
            || b[Pieces::Pawn as usize] > 0
            || self.has_bishop_pair(BLACK)
            || self.has_bishop_pair(WHITE)
            || w[Pieces::Knight as usize].count_ones() >= 3
            || b[Pieces::Knight as usize].count_ones() >= 3
            || (w[Pieces::Knight as usize].count_ones() >= 1 && w[Pieces::Bishop as usize] >= 1)
            || (b[Pieces::Knight as usize].count_ones() >= 1 && b[Pieces::Bishop as usize] >= 1))
    }
    fn has_bishop_pair(&self, side: u8) -> bool {
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
    fn get_piece_bb(&self, side: u8, piece: Pieces) -> u64 {
        self.pieces[side as usize][piece as usize]
    }
    pub fn get_bitboards(&self, side: u8) -> [u64; NrOf::PIECE_TYPES] {
        self.pieces[side as usize]
    }
}
#[cfg(test)]
mod tests {
    use crate::board::Pieces;
    use crate::defs::*;
    use crate::Board;

    #[test]
    fn test_init_side_bb() {
        let mut board = Board::new();
        board.pieces[WHITE as usize][Pieces::Pawn as usize] = 0x0FF00000000;
        board.init_side_bb();
        assert_eq!(board.side[WHITE as usize], 0x0FF00000000);
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
        board.pieces[WHITE as usize][Pieces::Bishop as usize] = 0b01100000000u64;
        board.pieces[BLACK as usize][Pieces::Bishop as usize] = 0b0000000010u64;
        assert!(board.has_bishop_pair(WHITE));
        assert!(!board.has_bishop_pair(BLACK));
        board.pieces[WHITE as usize][Pieces::Bishop as usize] = 0b000110010010u64;
        board.pieces[BLACK as usize][Pieces::Bishop as usize] = 0b000000000u64;
        assert!(board.has_bishop_pair(WHITE));
        assert!(!board.has_bishop_pair(BLACK));
        board.pieces[WHITE as usize][Pieces::Bishop as usize] = 0b000010000u64;
        board.pieces[BLACK as usize][Pieces::Bishop as usize] = 0b000110010010u64;
        assert!(!board.has_bishop_pair(WHITE));
        assert!(board.has_bishop_pair(BLACK));
    }
    #[test]
    fn test_has_sufficent_material() {
        let board = Board::fen(None);
        board.debug_bb();
        assert!(!board.draw_by_insufficient_material());
        let board = Board::fen(Some("8/8/8/5k2/8/7Q/1K6/8 w - - 0 1"));
        assert!(!board.draw_by_insufficient_material());
        let board = Board::fen(Some("8/8/3b4/5k2/8/8/1K6/8 w - - 0 1"));
        assert!(board.draw_by_insufficient_material());
    }
}
