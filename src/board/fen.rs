use super::*;
use crate::defs::*;
use core::fmt;
use std::fmt::Display;

pub enum FenError {
    IncorrectLength,
    Part1,
    Part2,
    Part3,
    Part4,
    Part5,
    Part6,
}

impl Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::IncorrectLength => "Error in Fen String: Must be 6 parts.",
            Self::Part1 => "Error in Fen String: Pieces or squares",
            Self::Part2 => "Error in Fen String: Colors",
            Self::Part3 => "Error in Fen String: Castling rights",
            Self::Part4 => "Error in Fen String: En passant field",
            Self::Part5 => "Error in Fen String: Half-Move clock",
            Self::Part6 => "Error in Fen String: Full move number",
        };
        write!(f, "{error}")
    }
}

const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";
const VALID_SYMBOLS_PIECES_AND_SQUARES: &str = "kqrbnpKQRBNP/0123456789";
const WHITE_AND_BLACK: &str = "wb";
const SPLITTER: char = '/';
const DASH: char = '-';
const SPACE: char = ' ';
type Parser = fn(&mut Board, &str) -> Result<(), FenError>;

pub fn split_fen_string(fen: &str) -> Result<Vec<String>, FenError> {
    let mut parts: Vec<String> = fen.split(SPACE).map(String::from).collect();
    if parts.len() == SHORT_FEN_LENGTH {
        parts.append(&mut vec![String::from('0'), String::from('1')]);
    }
    if parts.len() != FEN_LENGTH {
        return Err(FenError::IncorrectLength);
    }
    Ok(parts)
}

pub fn color(board: &mut Board, part: &str) -> Result<(), FenError> {
    if part.is_empty() || part.len() != 1 {
        return Err(FenError::Part2);
    }
    let c = match part.chars().next() {
        Some(i) => i,
        _ => return Err(FenError::Part2),
    };
    if !WHITE_AND_BLACK.contains(c) {
        return Err(FenError::Part2);
    }
    if let 'w' = c {
        board.game_state.active_color = WHITE as u8;
    } else {
        board.game_state.active_color = BLACK as u8;
    }
    Ok(())
}

pub fn castling(board: &mut Board, part: &str) -> Result<(), FenError> {
    // accepts lengths from 1 to 4
    if part.is_empty() || part.len() > 4 {
        print!("Error: INVALID LENGHT of CASTLING RIGHTS");
        return Err(FenError::Part3);
    }

    for c in part.chars() {
        match c {
            'K' => board.game_state.castling |= Castling::WK,
            'Q' => board.game_state.castling |= Castling::WQ,
            'k' => board.game_state.castling |= Castling::BK,
            'q' => board.game_state.castling |= Castling::BQ,
            '-' => (),
            _ => return Err(FenError::Part3),
        }
    }
    Ok(())
}

pub fn en_passant(board: &mut Board, part: &str) -> Result<(), FenError> {
    if part.len() == 1 {
        if let Some(x) = part.chars().next() {
            if x == DASH {
                board.game_state.en_passant = None;
                return Ok(());
            }
        }
    } else if part.len() == 2 {
        let square = algebraic_to_square(part);
        if let Some(sq) = square {
            if !(EN_PASSANT_SQUARES_WHITE.contains(&sq) || EN_PASSANT_SQUARES_BLACK.contains(&sq)) {
                return Err(FenError::Part4);
            }
        }
        board.game_state.en_passant = square;
        return Ok(());
    }
    Err(FenError::Part4)
}

pub fn half_move_counter(board: &mut Board, part: &str) -> Result<(), FenError> {
    if part.is_empty() || part.len() > 4 {
        return Err(FenError::Part5);
    }
    if let Ok(x) = part.parse::<u16>() {
        if x <= MAX_MOVE_RULE as u16 {
            board.game_state.halfmove_clock = x;
            return Ok(());
        }
    }
    Err(FenError::Part5)
}

pub fn full_move_counter(board: &mut Board, part: &str) -> Result<(), FenError> {
    if part.is_empty() || part.len() > 4 {
        return Err(FenError::Part6);
    }
    let trimmed = part.trim();
    if let Ok(x) = trimmed.parse::<u16>() {
        if x <= MAX_GAME_MOVES as u16 {
            board.game_state.fullmove_number = x;
            return Ok(());
        }
    }
    Err(FenError::Part6)
}

pub fn pieces(board: &mut Board, part: &str) -> Result<(), FenError> {
    if part.is_empty() {
        println!("Error: No String provided.");
        return Err(FenError::Part1);
    }
    // start at top left corner(square = 56)
    let mut file: u8 = 0;
    let mut rank: u8 = 7;
    let mut square;
    let mut result = true;
    let mut bitmask: u64 = 0;
    for c in part.chars() {
        square = (rank * 8) + file;
        // may be 64 after placing a piece on 63, but must be followed by a '/'
        if file == 8 && c != '/' {
            println!("Error: Square > 63 and no '/'");
            return Err(FenError::Part1);
        }

        if !VALID_SYMBOLS_PIECES_AND_SQUARES.contains(c) {
            println!("Error: Invalid Symbol.");
            return Err(FenError::Part1);
        }
        if c.is_alphabetic() {
            bitmask = helper::get_bitmask(square);
        }

        match c {
            // shift a 1 to the respective digit in the bitboard
            'K' => {
                board.pieces[WHITE as usize][Pieces::King as usize] |= bitmask;
            }
            'k' => {
                board.pieces[BLACK as usize][Pieces::King as usize] |= bitmask;
            }
            'Q' => {
                board.pieces[WHITE as usize][Pieces::Queen as usize] |= bitmask;
            }
            'q' => {
                board.pieces[BLACK as usize][Pieces::Queen as usize] |= bitmask;
            }
            'R' => {
                board.pieces[WHITE as usize][Pieces::Rook as usize] |= bitmask;
            }
            'r' => {
                board.pieces[BLACK as usize][Pieces::Rook as usize] |= bitmask;
            }
            'B' => {
                board.pieces[WHITE as usize][Pieces::Bishop as usize] |= bitmask;
            }
            'b' => {
                board.pieces[BLACK as usize][Pieces::Bishop as usize] |= bitmask;
            }
            'N' => {
                board.pieces[WHITE as usize][Pieces::Knight as usize] |= bitmask;
            }
            'n' => {
                board.pieces[BLACK as usize][Pieces::Knight as usize] |= bitmask;
            }
            'P' => {
                board.pieces[WHITE as usize][Pieces::Pawn as usize] |= bitmask;
            }
            'p' => {
                board.pieces[BLACK as usize][Pieces::Pawn as usize] |= bitmask;
            }
            '/' => {
                // TODO: add a check to not have '//' be possible
                // TODO: clean up the rank == 0 check
                if rank > 0 && file == 8 {
                    file = 0;
                    rank -= 1;
                } else {
                    println!("Error: RANK OVERFLOW or FILE NOT 8");
                    result = false;
                }
            }
            '1'..='8' => {
                if let Some(i) = c.to_digit(10) {
                    // using -1 to account for 0 based index
                    if (file + (i) as u8) <= 8 {
                        file += (i) as u8;
                    } else {
                        println!("Error: FILE OVERFLOW");
                        result = false;
                    }
                }
            }
            _ => {
                println!("Error: Unkown character");
                result = false;
            }
        }
        if LIST_OF_PIECES.contains(c) {
            file += 1;
        }

        if !result {
            return Err(FenError::Part1);
        }
    }
    // fen strings set h1=7 last
    if rank == 0 && file == 8 {
        Ok(())
    } else {
        println!("Error: did not correctly SET ALL SQUARES.");
        Err(FenError::Part1)
    }
}

pub fn create_fen_parsers() -> Vec<Parser> {
    vec![
        pieces,
        color,
        castling,
        en_passant,
        half_move_counter,
        full_move_counter,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_castling() {
        let mut board = Board::new();
        assert!(castling(&mut board, "KQkq").is_ok());
        assert_eq!(
            board.game_state.castling,
            Castling::WK | Castling::WQ | Castling::BK | Castling::BQ
        );

        let mut board = Board::new();
        assert!(castling(&mut board, "KQ").is_ok());
        assert_eq!(board.game_state.castling, Castling::WK | Castling::WQ);

        let mut board = Board::new();
        assert!(castling(&mut board, "kq").is_ok());
        assert_eq!(board.game_state.castling, Castling::BK | Castling::BQ);

        let mut board = Board::new();
        assert!(castling(&mut board, "-").is_ok());
        assert_eq!(board.game_state.castling, 0);
    }

    #[test]
    fn test_invalid_castling() {
        let mut board = Board::new();
        assert!(castling(&mut board, "ABC").is_err());

        let mut board = Board::new();
        assert!(castling(&mut board, "KQkqX").is_err()); // 'X' is invalid

        let mut board = Board::new();
        assert!(castling(&mut board, "").is_err()); // Empty input
    }

    #[test]
    fn test_color_valid() {
        let mut board = Board::new();
        assert!(color(&mut board, "w").is_ok());
        assert_eq!(board.game_state.active_color, WHITE as u8);

        let mut board = Board::new();
        assert!(color(&mut board, "b").is_ok());
        assert_eq!(board.game_state.active_color, BLACK as u8);
    }

    #[test]
    fn test_color_invalid() {
        let mut board = Board::new();
        assert!(color(&mut board, "").is_err());

        let mut board = Board::new();
        assert!(color(&mut board, "-").is_err());
    }

    // pub fn half_move_counter(board: &mut Board, part: &str) -> Result<(), FenError> {

    #[test]
    fn test_half_move_counter_valid() {
        let mut board = Board::new();
        assert!(half_move_counter(&mut board, "10").is_ok());
        assert_eq!(board.game_state.halfmove_clock, 10);

        // if x > 100 => collides with MAX_MOVE_RULE=100
        let mut board = Board::new();
        assert!(half_move_counter(&mut board, "100").is_ok());
        assert_eq!(board.game_state.halfmove_clock, 100);
    }

    #[test]
    fn test_half_move_counter_invalid() {
        let mut board = Board::new();
        assert!(half_move_counter(&mut board, "").is_err());

        let mut board = Board::new();
        assert!(half_move_counter(&mut board, "-10").is_err());

        let mut board = Board::new();
        assert!(half_move_counter(&mut board, "-").is_err());

        let mut board = Board::new();
        assert!(half_move_counter(&mut board, "abc").is_err());
    }

    #[test]
    fn test_full_move_counter_valid() {
        let mut board = Board::new();
        assert!(full_move_counter(&mut board, "10").is_ok());
        assert_eq!(board.game_state.fullmove_number, 10);

        let mut board = Board::new();
        assert!(full_move_counter(&mut board, "100").is_ok());
        assert_eq!(board.game_state.fullmove_number, 100);

        // x < 1024=MAX_GAME_MOVES
        let mut board = Board::new();
        assert!(full_move_counter(&mut board, "999").is_ok());
        assert_eq!(board.game_state.fullmove_number, 999);
    }

    #[test]
    fn test_full_move_counter_invalid() {
        let mut board = Board::new();
        assert!(full_move_counter(&mut board, "").is_err());

        let mut board = Board::new();
        assert!(full_move_counter(&mut board, "-10").is_err());

        let mut board = Board::new();
        assert!(full_move_counter(&mut board, "-").is_err());

        let mut board = Board::new();
        assert!(full_move_counter(&mut board, "abc").is_err());
    }

    #[test]
    fn test_valid_pieces() {
        let mut board = Board::new();

        // all numbers
        assert!(pieces(&mut board, "8/8/8/8/8/8/8/8").is_ok());
        // all pieces
        assert!(pieces(
            &mut board,
            "pppppppp/pppppppp/pppppppp/pppppppp/pppppppp/pppppppp/pppppppp/pppppppp"
        )
        .is_ok());

        // numbers with pieces in the middle
        assert!(pieces(&mut board, "1r6/1r6/1r6/1r6/1r6/1r6/1r6/1r6").is_ok());

        // pieces with numbers in the middle
        assert!(pieces(&mut board, "n5pr/n5pr/n5pr/n5pr/n5pr/n5pr/n5pr/n5pr").is_ok());
        assert!(pieces(
            &mut board,
            "rnbqkb1r/pp1p1ppp/5n2/2pPp3/2P5/8/PP2PPPP/RNBQKBNR"
        )
        .is_ok());

        board = Board::new();
        assert!(pieces(
            &mut board,
            "2kr1bnr/ppq1p1pp/2p5/n2p1b2/1PP1PpP1/2NP1N1P/P3P3/R1BQKB1R"
        )
        .is_ok());

        board = Board::new();
        assert!(pieces(&mut board, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").is_ok());

        board = Board::new();
        assert!(pieces(
            &mut board,
            "r1bqk1nr/pppp1ppp/2n5/2b5/3NP3/8/PPP2PPP/RNBQKB1R"
        )
        .is_ok());

        board = Board::new();
        assert!(pieces(&mut board, "8/8/8/5k2/8/7Q/1K6/8").is_ok());
    }

    #[test]
    fn test_invalid_pieces() {
        let mut board = Board::new();
        assert!(pieces(&mut board, "").is_err()); // Empty input

        let mut board = Board::new();
        // '9' is invalid
        assert!(pieces(&mut board, "rnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR").is_err());

        let mut board = Board::new();
        // Consecutive '/' is invalid
        assert!(pieces(&mut board, "rnbqkbnr/pppppppp/8//8/8/PPPPPPPP/RNBQKBNR").is_err());

        let mut board = Board::new();
        // exceeding board size by height
        assert!(pieces(&mut board, "rnbqkbnr/pppppppp/9/8/8/8/8/PPPPPPPP/RNBQKBNR").is_err());

        let mut board = Board::new();
        // not all squares have been set
        assert!(pieces(&mut board, "rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR").is_err());

        let mut board = Board::new();
        // exceeding board size by width
        assert!(pieces(&mut board, "rnbqkbnrr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR").is_err());
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
    fn test_en_passant() {
        let mut board = Board::new();
        assert!(en_passant(&mut board, "-").is_ok());
        assert_eq!(board.game_state.en_passant, None);
        assert!(en_passant(&mut board, "e3").is_ok());
        assert_eq!(
            board.game_state.en_passant,
            Some(algebraic_to_square("e3").unwrap())
        );
        assert!(en_passant(&mut board, "b3").is_ok());
        assert_eq!(
            board.game_state.en_passant,
            Some(algebraic_to_square("b3").unwrap())
        );
        assert!(en_passant(&mut board, "a3").is_ok());
        assert_eq!(
            board.game_state.en_passant,
            Some(algebraic_to_square("a3").unwrap())
        );
        assert!(en_passant(&mut board, "e6").is_ok());
        assert_eq!(
            board.game_state.en_passant,
            Some(algebraic_to_square("e6").unwrap())
        );

        assert!(en_passant(&mut board, "").is_err());
        assert!(en_passant(&mut board, "a1").is_err());
        assert!(en_passant(&mut board, "c8").is_err());
        assert!(en_passant(&mut board, "b5").is_err());
    }
}
