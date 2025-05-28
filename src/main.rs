mod board;
mod defs;
mod helper;
mod move_generator;

use board::*;
use defs::*;
use moves::{
    Castle::{Kingside, Queenside},
    Move, MoveType,
};
use pieces::Pieces;

fn main() {
    println!("Hello, world!");
    // let mut board = Board::fen(Some("8/1k1K4/8/8/8/8/8/Rn6 w - - 0 1"));
    // let capture: Move = Move::new(Pieces::Rook, 0, 1, MoveType::Capture(Pieces::Knight));
    let mut board = Board::fen(Some("1k6/8/8/8/8/8/8/R3K2R w KQ - 0 1"));
    let castle_white_queenside: Move = Move::new(Pieces::King, 4, 2, MoveType::Castle(Queenside));
    board.display();
    board.make(castle_white_queenside);
    println!("Making move");
    board.display();
    println!("Unmaking move");
    board.unmake();
    board.display();
}
