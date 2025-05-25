use crate::moves::{
    self,
    Castle::{Kingside, Queenside},
    Move, MoveType,
};
use crate::{Squares, BLACK, WHITE};

use super::{
    gamestate::{self, GameState},
    get_bitmask,
};
use super::{
    Board, Pieces, Square, EN_PASSANT_END_SQUARES_BLACK, EN_PASSANT_END_SQUARES_WHITE,
    EN_PASSANT_START_SQUARES_BLACK, EN_PASSANT_START_SQUARES_WHITE,
};

impl Board {
    pub fn remove_piece(&mut self, side: u8, piece: Pieces, square: Square) {
        let mask: u64 = get_bitmask(square);
        self.pieces[side as usize][piece as usize] ^= mask;
        self.side[side as usize] ^= mask;
        self.piece_list[square as usize] = Pieces::Empty;
        self.game_state.zobrist_key ^= self.zobrist_randoms.pieces(side, piece, square);
    }

    pub fn put_piece(&mut self, side: u8, piece: Pieces, square: Square) {
        let mask: u64 = get_bitmask(square);
        self.pieces[side as usize][piece as usize] ^= mask;
        self.side[side as usize] ^= mask;
        self.piece_list[square as usize] = piece;
        self.game_state.zobrist_key ^= self.zobrist_randoms.pieces(side, piece, square);
    }

    pub fn regular_move(&mut self, side: u8, piece: Pieces, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.put_piece(side, piece, to);
    }

    pub fn set_ep_square(&mut self, square: Square) {
        // Updates happening here
        self.game_state.en_passant = Some(square);
        // Updates happening here
    }
    pub fn clear_ep_square(&mut self) {
        // Updates happening here
        self.game_state.en_passant = None;
        // Updates happening here
    }
    pub fn switch_side(&mut self) {
        // Updates happening here
        self.game_state.active_color ^= 1;
        // Updates happening here
    }
    pub fn update_castling_permissions(&mut self, new_permissions: u8) {
        // Updates happening here
        self.game_state.castling = new_permissions;
        // Updates happening here
    }
    pub fn unmake(&mut self) {
        // go back in history by one half move(i think it's a half move)
        let previous_zobrist = self.game_state.zobrist_key;
        self.game_state = self.history.unmake();
        self.game_state.zobrist_key = previous_zobrist;
        // now unmake the move
        let move_to_unmake = self.game_state.next_move;
        match move_to_unmake.kind() {
            MoveType::Regular => self.regular_move(
                self.we(),
                move_to_unmake.piece(),
                move_to_unmake.to(),
                move_to_unmake.from(),
            ),
            MoveType::Capture(piece) => {
                self.regular_move(
                    self.we(),
                    move_to_unmake.piece(),
                    move_to_unmake.to(),
                    move_to_unmake.from(),
                );
                self.put_piece(self.opponent(), piece, move_to_unmake.to())
            }

            MoveType::Promotion(piece) => {
                self.promotion_move(piece, move_to_unmake.to(), move_to_unmake.piece());
                self.regular_move(
                    self.we(),
                    Pieces::Pawn,
                    move_to_unmake.to(),
                    move_to_unmake.from(),
                );
            }
            MoveType::Castle(castle) => {
                // move rook back
                let rooks = get_rook_start_squares(&self.game_state);
                let kingside_rook = rooks[0];
                let queenside_rook = rooks[1];
                match castle {
                    Kingside => self.regular_move(
                        self.we(),
                        Pieces::Rook,
                        self.king(self.we()) - 1,
                        kingside_rook,
                    ),
                    Queenside => self.regular_move(
                        self.we(),
                        Pieces::Rook,
                        self.king(self.we()) + 1,
                        queenside_rook,
                    ),
                }
                // move king back
                self.regular_move(
                    self.we(),
                    move_to_unmake.piece(),
                    move_to_unmake.to(),
                    move_to_unmake.from(),
                );
            }
        };
    }
    pub fn make(&mut self, m: Move) {
        // create a respawn point
        self.game_state.next_move = m;
        self.history.push(self.game_state);
        self.game_state.next_move = Move::default();
        // try out the move
        let piece = m.piece();
        // actually move on the board
        self.game_state.halfmove_clock += 1;
        match m.kind() {
            MoveType::Regular => self.regular_move(self.we(), piece, m.from(), m.to()),
            MoveType::Capture(captured_piece) => {
                self.capture_move(piece, m.from(), m.to(), captured_piece);
                // resetting halfmove_clock
                self.game_state.halfmove_clock = 0;
            }
            MoveType::Promotion(promoted_piece) => {
                self.regular_move(self.we(), piece, m.from(), m.to());
                self.promotion_move(piece, m.to(), promoted_piece);
            }
            MoveType::Castle(direction) => self.castle(m.piece(), m.from(), m.to(), direction),
        }
        // swap side
        self.switch_side();
        // set/remove en_passant
        if piece == Pieces::Pawn {
            // resetting halfmove_clock
            self.game_state.halfmove_clock = 0;
            if EN_PASSANT_START_SQUARES_WHITE.contains(&m.from())
                || EN_PASSANT_START_SQUARES_BLACK.contains(&m.from())
                    && (EN_PASSANT_END_SQUARES_WHITE.contains(&m.to())
                        || EN_PASSANT_END_SQUARES_BLACK.contains(&m.to()))
            {
                // set ep square as square on same file in next rank
                self.set_ep_square(&m.from() + 8);
            } else {
                self.clear_ep_square();
            }
        }

        // assuming the move is valid, store the new position
        self.history.push(self.game_state);
        // TODO:
        // all validity checks:
        // - castling
        // - checks
        // - pins
    }
    fn capture_move(&mut self, piece: Pieces, from: Square, to: Square, captured_piece: Pieces) {
        // remove piece that will be captured
        self.remove_piece(self.opponent(), captured_piece, to);
        // move piece regularely
        self.regular_move(self.we(), piece, from, to);
    }

    fn promotion_move(&mut self, piece: Pieces, square: Square, promoted_piece: Pieces) {
        self.remove_piece(self.we(), piece, square);
        self.put_piece(self.we(), promoted_piece, square);
    }

    fn castle(&mut self, piece: Pieces, from: Square, to: Square, direction: moves::Castle) {
        // BUG: white kingside castling does not work

        // move king as per move
        self.regular_move(self.we(), piece, from, to);
        // move rook in relation to king
        let rooks = get_rook_start_squares(&self.game_state);
        let kingside_rook = rooks[0];
        let queenside_rook = rooks[1];
        match direction {
            Kingside => {
                self.regular_move(
                    self.we(),
                    Pieces::Rook,
                    kingside_rook,
                    self.king(self.we()) - 1,
                );
            }
            Queenside => {
                self.regular_move(
                    self.we(),
                    Pieces::Rook,
                    queenside_rook,
                    self.king(self.we()) + 1,
                );
            }
        }
    }

    fn swap_piece(&mut self, piece: Pieces, square: Square) {}
}

fn get_rook_start_squares(state: &GameState) -> [Square; 2] {
    let kingside_rook: Square;
    let queenside_rook: Square;
    match state.active_color {
        WHITE => {
            queenside_rook = Squares::A1;
            kingside_rook = Squares::H1;
        }
        BLACK => {
            queenside_rook = Squares::A8;
            kingside_rook = Squares::H8;
        }
        x => panic!(
            "Unknown side in active color. (expected 0 or 1 - found {})",
            x
        ),
    };
    [kingside_rook, queenside_rook]
}

#[cfg(test)]
mod tests {
    use crate::{
        moves::{
            Castle::{Kingside, Queenside},
            Move, MoveType,
        },
        pieces::Pieces,
        Board,
    };

    #[test]
    fn test_unmake() {
        // REGULAR moves
        // Pawn En-Passnt
        let mut board = Board::fen(None);
        let mut board_copy = board.clone();
        let m: Move = Move::new(Pieces::Pawn, 12, 28, MoveType::Regular);
        board_copy.game_state.next_move = m;
        board.make(m);
        board.unmake();
        assert_eq!(board_copy, board);

        // PAWN
        board = Board::fen(None);
        board_copy = board.clone();
        let m: Move = Move::new(Pieces::Pawn, 12, 20, MoveType::Regular);
        board_copy.game_state.next_move = m;
        board.make(m);
        board.unmake();
        assert_eq!(board_copy, board);

        // KNIGHT
        board = Board::fen(None);
        board_copy = board.clone();
        let m: Move = Move::new(Pieces::Knight, 1, 18, MoveType::Regular);
        board_copy.game_state.next_move = m;
        board.make(m);
        board.unmake();
        assert_eq!(board_copy, board);

        // CAPTURE
        board = Board::fen(Some("8/8/4k3/8/K7/8/8/Rn6 w - - 0 1"));
        board_copy = board.clone();
        let m: Move = Move::new(Pieces::Rook, 0, 1, MoveType::Capture(Pieces::Knight));
        board_copy.game_state.next_move = m;
        board.make(m);
        board.unmake();
        assert_eq!(board_copy, board);

        // PROMOTION
        board = Board::fen(Some("k7/8/2K5/8/8/8/p7/8 w - - 0 1"));
        board_copy = board.clone();
        let m: Move = Move::new(Pieces::Pawn, 8, 0, MoveType::Promotion(Pieces::Queen));
        board_copy.game_state.next_move = m;
        board.make(m);
        board.unmake();
        assert_eq!(board_copy, board);

        // CASTLING
        // WHITE KINGSIDE
        board = Board::fen(Some("1k6/8/8/8/8/8/8/4K2R w K - 0 1"));
        board_copy = board.clone();
        let m: Move = Move::new(Pieces::King, 4, 6, MoveType::Castle(Kingside));
        board_copy.game_state.next_move = m;
        board.make(m);
        board.unmake();
        assert_eq!(board_copy, board);

        // WHITE QUEENSIDE
        board = Board::fen(Some("1k6/8/8/8/8/8/8/R3K3 w Q - 0 1"));
        board_copy = board.clone();
        let m: Move = Move::new(Pieces::King, 4, 2, MoveType::Castle(Queenside));
        board_copy.game_state.next_move = m;
        board.make(m);
        board.unmake();
        assert_eq!(board_copy, board);
    }
}
