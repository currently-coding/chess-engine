use crate::{helper::get_bitmask, moves::Move, NrOf};

pub struct MoveGenerator {
    pawn: [[u64; NrOf::SQUARES]; NrOf::SIDES],
    knight: [u64; NrOf::SQUARES],
    bishop: [u64; NrOf::SQUARES],
    rook: [u64; NrOf::SQUARES],
    king: [u64; NrOf::SQUARES],
    // queen is rook || bishop
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut mg = MoveGenerator {
            pawn: [[0; NrOf::SQUARES]; NrOf::SIDES],
            knight: [0; NrOf::SQUARES],
            bishop: [0; NrOf::SQUARES],
            rook: [0; NrOf::SQUARES],
            king: [0; NrOf::SQUARES],
        };
        mg.init_pawn();
        // mg.init_knight();
        mg.init_bisphop();
        mg.init_rook();
        mg.init_king();
        mg
    }

    fn init_pawn(&mut self) {
        let dirs: [(i8, i8); 2] = [(1, 0), (-1, 0)];
        for rank in 0..8 {
            for file in 0..8 {
                for (side, dir) in dirs.iter().enumerate() {
                    let square = (rank * 8 + file) as u8;
                    let new_rank = rank + dir.0;
                    let new_file = file + dir.1; // not necessary
                    if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
                        let new_square = (new_rank * 8 + new_file) as u8;
                        let bitmask = get_bitmask(new_square);
                        self.pawn[side][square as usize] |= bitmask;
                    }
                }
            }
        }
    }

    fn init_bisphop(&self) {
        todo!()
    }

    fn init_rook(&self) {
        todo!()
    }

    fn init_king(&mut self) {
        let dirs: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for rank in 0..8 {
            for file in 0..8 {
                for dir in dirs {
                    let square = ((rank * 8) + file) as u8;
                    let new_rank = rank + dir.0;
                    let new_file = file + dir.1;
                    if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
                        let new_square = (new_rank * 8 + new_file) as u8;
                        let bitmask = get_bitmask(new_square);
                        self.king[square as usize] |= bitmask;
                    }
                }
            }
        }
    }
}
