use crate::helper::get_bitmask;

pub struct Bitboard(u64);

impl Bitboard {
    pub fn new(bb: u64) -> Self {
        Self(bb)
    }
}

impl Iterator for Bitboard {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.0 == 0 {
            return None;
        }
        let square: u8 = self.0.trailing_zeros() as u8;
        self.0 ^= get_bitmask(square);
        Some(square)
    }
}
