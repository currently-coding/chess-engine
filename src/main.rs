mod board;
mod cli;
mod defs;
mod helper;
mod move_generator;

use board::*;
use defs::*;
use move_generator::MoveGenerator;
use moves::{
    Castle::{Kingside, Queenside},
    Move, MoveType,
};
use pieces::Pieces;

fn main() {
    let mg = MoveGenerator::new();
    user();
    // testing();
}

fn testing() {
    println!("Hello, world!");
    // let mut board = Board::fen(Some("8/1k1K4/8/8/8/8/8/Rn6 w - - 0 1"));
    // let capture: Move = Move::new(Pieces::Rook, 0, 1, MoveType::Capture(Pieces::Knight));
    let mut board = Board::fen(Some("1k6/8/8/8/8/8/8/R3K2R w KQ - 0 1".to_string()));
    let castle_white_queenside: Move = Move::new(Pieces::King, 4, 2, MoveType::Castle(Queenside));
    board.display();
    board.game_state.debug();
    board.make(castle_white_queenside);
    println!("Making move");
    board.game_state.debug();
    board.display();
    println!("Unmaking move");
    board.unmake();
    board.display();
    board.game_state.debug();
}

fn user() {
    let fen: Option<String> = cli::get_fen();
    let mut board = Board::fen(fen);
    loop {
        board.display();
        let m: Move = cli::get_move();
        board.make(m);
    }
}
