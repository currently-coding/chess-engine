use crate::board::moves::Move;
use crate::board::pieces::Pieces;
use crate::defs::*;
use crate::{helper::get_bitmask, NrOf};

pub struct MoveGenerator {
    pawn: [[u64; NrOf::SQUARES]; NrOf::SIDES],
    knight: [u64; NrOf::SQUARES],
    bishop: [u64; NrOf::SQUARES],
    rook: [u64; NrOf::SQUARES],
    king: [u64; NrOf::SQUARES],
    // queen is rook || bishop
}

impl MoveGenerator {
    pub const KNIGHT_DIRS: [(i8, i8); 8] = [
        (-1, 2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (1, -2),
        (1, 2),
        (2, 1),
        (2, -1),
    ];
    pub const BISHOP_DIRS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    pub const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    pub const KING_DIRS: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    pub fn new() -> Self {
        let mut mg = MoveGenerator {
            pawn: [[0; NrOf::SQUARES]; NrOf::SIDES],
            knight: [0; NrOf::SQUARES],
            bishop: [0; NrOf::SQUARES],
            rook: [0; NrOf::SQUARES],
            king: [0; NrOf::SQUARES],
        };
        mg.init_pawn();
        mg.init_bisphop();
        mg.init_knight();
        mg.init_rook();
        mg.init_king();
        mg
    }
    pub fn get_moves(&self, side: u8, piece: &Pieces, square: u8) -> u64 {
        let side = side as usize;
        let square = square as usize;
        match piece {
            Pieces::Pawn => self.pawn[side][square],
            Pieces::Bishop => self.bishop[square],
            Pieces::Knight => self.knight[square],
            Pieces::Rook => self.rook[square],
            Pieces::Queen => self.rook[square] | self.bishop[square],
            Pieces::King => self.king[square],
            Pieces::Empty => panic!("Cannot generate moves for Piece \"Empty\"."),
        }
    }

    fn init_pawn(&mut self) {
        self.pawn[WHITE as usize] = get_attacks(vec![(1, 0), (1, -1), (1, 1)]);
        self.pawn[BLACK as usize] = get_attacks(vec![(-1, 0), (-1, -1), (-1, 1)]);
        for (square, bb) in self.pawn[WHITE as usize].iter_mut().enumerate() {
            if EN_PASSANT_START_SQUARES_WHITE.contains(&(square as u8)) {
                *bb |= get_bitmask((square + 16) as u8);
            }
        }
        for (square, bb) in self.pawn[BLACK as usize].iter_mut().enumerate() {
            if EN_PASSANT_START_SQUARES_BLACK.contains(&(square as u8)) {
                *bb |= get_bitmask((square - 16) as u8);
            }
        }
    }
    pub fn sliding_attacks(
        square: u8,
        directions: &[(i8, i8)],
        captures: u64,
        blockers: u64,
    ) -> u64 {
        let mut attacks = 0;
        let (rank, file) = (square / 8, square % 8);

        for &(dr, df) in directions {
            let mut r = rank as i8 + dr;
            let mut f = file as i8 + df;

            while (0..8).contains(&r) && (0..8).contains(&f) {
                let sq = (r * 8 + f) as u8;
                let bb = get_bitmask(sq);
                if blockers & bb != 0 {
                    break;
                }
                attacks |= bb;
                if captures & bb != 0 {
                    break; // blocked, stop here
                }
                r += dr;
                f += df;
            }
        }

        attacks
    }

    fn init_bisphop(&mut self) {
        self.bishop = get_slide_attacks(Vec::from(MoveGenerator::BISHOP_DIRS));
    }

    fn init_rook(&mut self) {
        self.rook = get_slide_attacks(Vec::from(MoveGenerator::ROOK_DIRS));
    }

    fn init_king(&mut self) {
        self.king = get_attacks(Vec::from(MoveGenerator::KING_DIRS));
        self.king[Squares::E1 as usize] |= get_bitmask(Squares::C1) | get_bitmask(Squares::G1);
        self.king[Squares::E8 as usize] |= get_bitmask(Squares::C8) | get_bitmask(Squares::G8);
    }

    fn init_knight(&mut self) {
        self.knight = get_attacks(Vec::from(MoveGenerator::KNIGHT_DIRS));
    }

    pub fn pawn(&self) -> [[u64; NrOf::SQUARES]; NrOf::SIDES] {
        self.pawn
    }

    pub fn knight(&self) -> [u64; NrOf::SQUARES] {
        self.knight
    }

    pub fn bishop(&self) -> [u64; NrOf::SQUARES] {
        self.bishop
    }

    pub fn rook(&self) -> [u64; NrOf::SQUARES] {
        self.rook
    }

    pub fn king(&self) -> [u64; NrOf::SQUARES] {
        self.king
    }
}
fn get_attacks(dirs: Vec<(i8, i8)>) -> [u64; 64] {
    let mut attacks = [0u64; 64];
    for rank in 0..8 {
        for file in 0..8 {
            for dir in &dirs {
                let square = ((rank * 8) + file) as u8;
                let new_rank = rank + dir.0;
                let new_file = file + dir.1;
                if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
                    let new_square = (new_rank * 8 + new_file) as u8;
                    let bitmask = get_bitmask(new_square);
                    attacks[square as usize] |= bitmask;
                }
            }
        }
    }
    attacks
}

fn get_slide_attacks(dirs: Vec<(i8, i8)>) -> [u64; 64] {
    let mut attacks = [0u64; 64];
    for rank in 0..8 {
        for file in 0..8 {
            for dir in &dirs {
                let square = (rank * 8 + file) as u8;
                let mut new_rank = rank + dir.0;
                let mut new_file = file + dir.1;
                loop {
                    if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
                        let current_square = (new_rank * 8 + new_file) as u8;
                        new_rank += dir.0;
                        new_file += dir.1;
                        let bitmask = get_bitmask(current_square);
                        attacks[square as usize] |= bitmask;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    attacks
}
