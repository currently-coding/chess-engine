use crate::defs::{self, NrOf};

pub fn get_bitmask(square: u8) -> u64 {
    if square >= NrOf::SQUARES as u8 {
        panic!(
            "get_bitmask received a square outside the board bounds: square={}",
            square
        )
    }
    1u64 << square
}

pub fn next_bit(bitboard: &mut u64) -> Option<u8> {
    // TODO: might be inefficient
    if *bitboard == 0 {
        return None;
    }
    // find first active bit
    let square: u8 = bitboard.trailing_zeros() as u8;
    // remove that bit - mut ref so it actually changes
    *bitboard ^= get_bitmask(square);
    // return the index of the found bit
    Some(square)
}

pub fn algebraic_to_square(location: &str) -> Option<u8> {
    if location.len() != 2 {
        return None;
    }

    let mut chars = location.chars();
    let file = chars.next().unwrap();
    let rank_char = chars.next().unwrap();

    if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank_char) {
        return None;
    }

    let file_num = file as u8 - b'a'; // Convert 'a'-'h' to 0-7
    let rank_num = (rank_char as u8 - b'1') * 8; // Convert '1'-'8' to 0-7 and scale

    Some(rank_num + file_num)
}

pub fn square_num_to_algebraic<'a>(square: u8) -> &'a str {
    defs::SQUARE_NAME[square as usize]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_square_num_to_algebraic() {
        assert_eq!(square_num_to_algebraic(63), "h8");
        assert_eq!(square_num_to_algebraic(0), "a1");
        assert_eq!(square_num_to_algebraic(9), "b2");
    }
    #[test]
    fn test_algebraic_to_square_num() {
        assert!(algebraic_to_square("a1").is_some());
        assert!(algebraic_to_square("h8").is_some());
        assert!(algebraic_to_square("e5").is_some());

        assert_eq!(algebraic_to_square("a1"), Some(0));
        assert_eq!(algebraic_to_square("h8"), Some(63));
        assert_eq!(algebraic_to_square("b2"), Some(9));
    }
    #[test]
    fn test_get_bitmask() {
        let mut square = 0;
        assert_eq!(
            get_bitmask(square),
            0b0000000000000000000000000000000000000000000000000000000000000001u64
        );
        square = 1;
        assert_eq!(
            get_bitmask(square),
            0b0000000000000000000000000000000000000000000000000000000000000010u64
        );
        square = 63;
        assert_eq!(
            get_bitmask(square),
            0b1000000000000000000000000000000000000000000000000000000000000000u64
        );
        square = 62;
        assert_eq!(
            get_bitmask(square),
            0b0100000000000000000000000000000000000000000000000000000000000000u64
        );
        square = 50;
        assert_eq!(
            get_bitmask(square),
            0b0000000000000100000000000000000000000000000000000000000000000000u64
        );
    }
    #[test]
    fn test_next_bit() {
        let mut bitboard: u64 = 0b10100; // Example bitboard with bits set at positions 2 and 4
        let first_bit = next_bit(&mut bitboard);
        assert_eq!(first_bit, Some(2)); // The first active bit is at position 2
        assert_eq!(bitboard, 0b10000); // After removing the bit, bitboard should be 0b10000

        let second_bit = next_bit(&mut bitboard);
        assert_eq!(second_bit, Some(4)); // The next active bit is at position 4
        assert_eq!(bitboard, 0b00000); // After removing the bit, bitboard should be 0b00000

        // Test when there are no active bits
        let mut empty_bitboard: u64 = 0b00000;
        let result = next_bit(&mut empty_bitboard);
        assert_eq!(result, None); // Should return 0, but this may depend on your implementation
        assert_eq!(empty_bitboard, 0b00000); // Bitboard should remain unchanged
    }
}
