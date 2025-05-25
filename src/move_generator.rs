// use crate::{moves::Move, NrOf};
//
// pub struct MoveGenerator {
//     pawn: [[u64; NrOf::SQUARES]; NrOf::SIDES],
//     knight: [u64; NrOf::SQUARES],
//     bishop: [u64; NrOf::SQUARES],
//     rook: [u64; NrOf::SQUARES],
//     king: [u64; NrOf::SQUARES],
//     // queen is rook || bishop
// }
//
// impl MoveGenerator {
//     pub fn new() -> Self {
//         let mg = MoveGenerator {
//             pawn: [[0; NrOf::SQUARES]; NrOf::SIDES],
//             knight: [0; NrOf::SQUARES],
//             bishop: [0; NrOf::SQUARES],
//             rook: [0; NrOf::SQUARES],
//             king: [0; NrOf::SQUARES],
//         };
//         mg.init_pawn();
//         mg.init_knight();
//         mg.init_bisphop();
//         mg.init_rook();
//         mg.init_king();
//     }
//
//     fn init_pawn(&self) {
//         todo!()
//     }
//
//     fn init_knight(&self) -> _ {
//         let bb = [0u64; NrOf::SQUARES];
//         for (index, square) in bb.iter_mut().enumerate() {
//             square = 0u64 << index;
//             // TODO: manipulate to store actual possible moves
//         }
//
//         todo!()
//     }
//
//     fn init_bisphop(&self) -> _ {
//         todo!()
//     }
//
//     fn init_rook(&self) -> _ {
//         todo!()
//     }
//
//     fn init_king(&self) -> _ {
//         todo!()
//     }
// }
