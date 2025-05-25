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
    let mut board = Board::fen(Some(
        "rnbqkbnr/pp2pppp/2p5/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 1",
    ));
    let m: Move = Move::new(
        Pieces::King,
        board.king(board.opponent()),
        board.king(board.opponent()) + 2,
        MoveType::Castle(Kingside),
    );
    board.display();
    board.make(m);
    println!("Making move");
    board.display();
    // println!("Unmaking move");
    // board.unmake();
    // board.display();
}
