mod board;
mod defs;
mod helper;

use board::*;
use defs::{Square, SQUARE_NAME};
use pieces::Pieces;
use sides::Sides;

fn main() {
    println!("Hello, world!");
    let mut board = Board::fen(Some("1K6/3b4/1q3p2/3n4/r7/4NQ2/2R5/6k1 w - - 0 1"));
    board.display();
    let mut from: Square = 8;
    let mut to: Square = 9;
    board.move_piece(Sides::White, Pieces::Pawn, 12, 20);
    board.debug();
    board.display();
}
