// use core::panic;
//
// #[repr(usize)]
// #[derive(Copy, Clone)]
// pub enum Sides {
//     White = 0,
//     Black = 1,
//     Both = 2,
// }
//
// impl Sides {
//     pub fn from(num: u8) -> Sides {
//         match num.into() {
//             0 => Sides::White,
//             1 => Sides::Black,
//             2 => Sides::Both,
//             _ => panic!("Unknown Side"),
//         }
//     }
// }
//
// impl From<Sides> for usize {
//     fn from(side: Sides) -> Self {
//         side as usize
//     }
// }
//
// impl From<usize> for Sides {
//     fn from(value: usize) -> Self {
//         match value {
//             0 => Sides::White,
//             1 => Sides::Black,
//             2 => Sides::Both,
//             _ => panic!("Unknown Sides value"),
//         }
//     }
// }
