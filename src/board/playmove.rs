use moves::MoveType;

use super::*;

impl Board {
    pub fn remove_piece(&mut self, side: Sides, piece: Pieces, square: Square) {
        let mask: u64 = get_bitmask(square);
        self.pieces[side as usize][piece as usize] ^= mask;
        self.side[side as usize] ^= mask;
        self.piece_list[square as usize] = Pieces::Empty;
        self.game_state.zobrist_key ^= self.zobrist_randoms.pieces(side, piece, square);
        // Updates happening here
    }

    pub fn put_piece(&mut self, side: Sides, piece: Pieces, square: Square) {
        let mask: u64 = get_bitmask(square);
        self.pieces[side as usize][piece as usize] ^= mask;
        self.side[side as usize] ^= mask;
        self.piece_list[square as usize] = piece;
        self.game_state.zobrist_key ^= self.zobrist_randoms.pieces(side, piece, square);
    }

    pub fn move_piece(&mut self, side: Sides, piece: Pieces, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.put_piece(side, piece, to);
    }
    pub fn set_ep_square(&mut self, square: Square) {
        // Updates happening here
        self.game_state.en_passant = Some(square as u8)
        // Updates happening here
    }
    pub fn clear_ep_square(&mut self, square: Square) {
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
    pub fn unmake(&mut self, moves: u8) {
        // go back in history by one half move(i think it's a half move)
        self.game_state = self.history.unmake();
        // now unmake the move
        let move_to_unmake = self.game_state.next_move;
        match move_to_unmake.kind {
            MoveType::Regular => self.move_piece(
                Sides::from(self.we() as u8),
                move_to_unmake.piece,
                move_to_unmake.to,
                move_to_unmake.from,
            ),
            MoveType::Capture(piece) => todo!(),
            MoveType::Promotion(piece) => todo!(),
            MoveType::Castle(castle) => todo!(),
        };
    }
}
