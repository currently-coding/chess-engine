use std::io::{self, Write};

pub(crate) fn get_fen() -> Option<String> {
    let mut input = String::new();
    println!("Enter FEN: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input.");
    input = input.trim().to_string();
    println!("You entered: -{}-", input);
    if input.is_empty() {
        println!("Defaulting to starting position.");
        None
    } else {
        Some(input)
    }
}

use crate::moves::{Castle, Move, MoveType};
use crate::pieces::Pieces;

fn square_to_index(sq: &str) -> u8 {
    let bytes = sq.as_bytes();
    (bytes[0] - b'a') + 8 * (bytes[1] - b'1')
}

pub(crate) fn get_move() -> Move {
    print!("Enter move: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let parts: Vec<&str> = input.split_whitespace().collect();

    let piece = match parts[0] {
        "P" => Pieces::Pawn,
        "N" => Pieces::Knight,
        "B" => Pieces::Bishop,
        "R" => Pieces::Rook,
        "Q" => Pieces::Queen,
        "K" => Pieces::King,
        _ => panic!("Unknown piece"), // no validity check requested
    };

    let from = square_to_index(parts[1]);
    let to = square_to_index(parts[2]);

    let kind = if parts.len() == 3 {
        MoveType::Regular
    } else if parts[3] == "capture" {
        let captured = match parts[4] {
            "P" => Pieces::Pawn,
            "N" => Pieces::Knight,
            "B" => Pieces::Bishop,
            "R" => Pieces::Rook,
            "Q" => Pieces::Queen,
            "K" => Pieces::King,
            _ => panic!("Unknown piece"),
        };
        MoveType::Capture(captured)
    } else if parts[3] == "promotion" {
        let promo = match parts[4] {
            "N" => Pieces::Knight,
            "B" => Pieces::Bishop,
            "R" => Pieces::Rook,
            "Q" => Pieces::Queen,
            _ => panic!("Unknown promotion"),
        };
        MoveType::Promotion(promo)
    } else if parts[3] == "castle" {
        let side = match parts[4] {
            "kingside" => Castle::Kingside,
            "queenside" => Castle::Queenside,
            _ => panic!("Unknown castling side"),
        };
        MoveType::Castle(side)
    } else {
        MoveType::Regular
    };

    Move::new(piece, from, to, kind)
}
