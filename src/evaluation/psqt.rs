/* =======================================================================
Rustic is a chess playing engine.
Copyright (C) 2019-2024, Marcel Vanthoor
https://rustic-chess.org/

Rustic is written in the Rust programming language. It is an original
work, not derived from any engine that came before it. However, it does
use a lot of concepts which are well-known and are in use by most if not
all classical alpha/beta-based chess engines.

Rustic is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License version 3 as published by
the Free Software Foundation.

Rustic is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
for more details.

You should have received a copy of the GNU General Public License along
with this program.  If not, see <http://www.gnu.org/licenses/>.
======================================================================= */

// This file implements Piece Square Tables (PSQT) for each piece type. The
// PSQT's are written from White's point of view, as if looking at a chess
// diagram, with A1 on the lower left corner.

use crate::{
    board::Board,
    defs::{NrOf, Sides},
    misc::bits,
};

type Psqt = [i16; NrOf::SQUARES];

#[rustfmt::skip]
const KING_MG: Psqt = [
    0, 0,  0,   0,   0, 0,  0, 0,
    0, 0,  0,   0,   0, 0,  0, 0,
    0, 0,  0,   0,   0, 0,  0, 0,
    0, 0,  0,  20,  20, 0,  0, 0,
    0, 0,  0,  20,  20, 0,  0, 0,
    0, 0,  0,   0,   0, 0,  0, 0,
    0, 0,  0, -10, -10, 0,  0, 0,
    0, 0, 20, -10, -10, 0, 20, 0,
];

#[rustfmt::skip]
const QUEEN_MG: Psqt = [
    870, 880, 890, 890, 890, 890, 880, 870,
    880, 890, 895, 895, 895, 895, 890, 880,
    890, 895, 910, 910, 910, 910, 895, 890,
    890, 895, 910, 920, 920, 910, 895, 890,
    890, 895, 910, 920, 920, 910, 895, 890,
    890, 895, 895, 895, 895, 895, 895, 890,
    880, 890, 895, 895, 895, 895, 890, 880,
    870, 880, 890, 890, 890, 890, 880, 870 
];

#[rustfmt::skip]
const ROOK_MG: Psqt = [
   500, 500, 500, 500, 500, 500, 500, 500,
   515, 515, 515, 520, 520, 515, 515, 515,
   500, 500, 500, 500, 500, 500, 500, 500,
   500, 500, 500, 500, 500, 500, 500, 500,
   500, 500, 500, 500, 500, 500, 500, 500,
   500, 500, 500, 500, 500, 500, 500, 500,
   500, 500, 500, 500, 500, 500, 500, 500,
   500, 500, 500, 510, 510, 510, 500, 500
];

#[rustfmt::skip]
const BISHOP_MG: Psqt = [
    300, 320, 320, 320, 320, 320, 320, 300,
    305, 320, 320, 320, 320, 320, 320, 305,
    310, 320, 320, 325, 325, 320, 320, 310,
    310, 330, 330, 350, 350, 330, 330, 310,
    325, 325, 330, 345, 345, 330, 325, 325,
    325, 325, 325, 330, 330, 325, 325, 325,
    310, 325, 325, 330, 330, 325, 325, 310,
    300, 310, 310, 310, 310, 310, 310, 300
];

#[rustfmt::skip]
const KNIGHT_MG: Psqt = [
    290, 300, 300, 300, 300, 300, 300, 290,
    300, 305, 305, 305, 305, 305, 305, 300,
    300, 305, 325, 325, 325, 325, 305, 300,
    300, 305, 325, 325, 325, 325, 305, 300,
    300, 305, 325, 325, 325, 325, 305, 300,
    300, 305, 325, 325, 325, 325, 305, 300,
    300, 305, 305, 305, 305, 305, 305, 300,
    290, 310, 300, 300, 300, 300, 310, 290
];

#[rustfmt::skip]
const PAWN_MG: Psqt = [
    100, 100, 100, 100, 100, 100, 100, 100,
    160, 160, 160, 160, 170, 160, 160, 160,
    140, 140, 140, 150, 160, 140, 140, 140,
    120, 120, 120, 140, 150, 120, 120, 120,
    105, 105, 115, 130, 140, 110, 105, 105,
    105, 105, 110, 120, 130, 105, 105, 105,
    105, 105, 105,  70,  70, 105, 105, 105,
    100, 100, 100, 100, 100, 100, 100, 100
];

pub const PSQT_MG: [Psqt; NrOf::PIECE_TYPES] =
    [KING_MG, QUEEN_MG, ROOK_MG, BISHOP_MG, KNIGHT_MG, PAWN_MG];

// When one side has a bare king, this PSQT is used to drive that king to
// the edge of the board and mate it there.
#[rustfmt::skip]
pub const KING_EDGE: Psqt = [
    -95,  -95,  -90,  -90,  -90,  -90,  -95,  -95,  
    -95,  -50,  -50,  -50,  -50,  -50,  -50,  -95,  
    -90,  -50,  -20,  -20,  -20,  -20,  -50,  -90,  
    -90,  -50,  -20,    0,    0,  -20,  -50,  -90,  
    -90,  -50,  -20,    0,    0,  -20,  -50,  -90,  
    -90,  -50,  -20,  -20,  -20,  -20,  -50,  -90,  
    -95,  -50,  -50,  -50,  -50,  -50,  -50,  -95,  
    -95,  -95,  -90,  -90,  -90,  -90,  -95,  -95,
];

// To make the Piece Square tables easier to relate to, and easier to
// edit, they have been laid out as a normal chess board, with A1 at
// the lower left. Because the square numbers start with A1 = 0, a
// conversion needs to be done. Below are the representations of the
// square numbers and the PSQT from both white and black point of view.

// (These tables will hold piece/square values instead of coordinates.
// The coordinates are used to visualize to which square a value in the
// table would belong.)

// Square numbers, as they are in arrays:

//  0  1  2  3  4  5  6  7   <= 0 = A1, 7 = H1
//  8  9 10 11 12 13 14 15
// 16 17 18 19 20 21 22 23
// 24 25 26 27 28 29 30 31
// 32 33 34 35 36 37 38 39
// 40 41 42 43 44 45 46 47
// 48 49 50 51 52 53 54 55
// 56 57 58 59 60 61 62 63  <= 56 = A8, 63 = H8

// PSQT, WHITE:                // Same PSQT, BLACK:

// A8 B8 C8 D8 E8 F8 G8 H8  |  // A1 B1 C1 D1 E1 G1 F1 H1
// A7 B7 C7 D8 E8 F8 G7 H7  |  // A2 B2 C2 D2 E2 G2 F2 H2
// A6 B6 C6 D6 E6 F6 G6 H6  |  // A3 B3 C3 D3 E3 G3 F3 H3
// A5 B5 C5 D5 E5 F5 G5 H5  |  // A4 B4 C4 D4 E4 G4 F4 H4
// A4 B4 C4 D4 E4 F4 G4 H4  |  // A5 B5 C5 D5 E5 G5 F5 H5
// A3 B3 C3 D3 E3 F3 G3 H3  |  // A6 B6 C6 D6 E6 F6 G6 H6
// A2 B2 C2 D2 E2 F2 G2 H2  |  // A7 B7 C7 D7 E7 G7 F7 H7
// A1 B1 C1 D1 E1 F1 G1 H1  |  // A8 B8 C8 D8 E8 F8 G8 H8

// If one super-imposes the square numbers on the PSQT with square names
// from WHITE's point of view, it can be seen that the following is true:

/*
    Square to PSQT element examples:
    A1 = square 0.  ==> PSQT element 56.
    H8 = square 63. ==> PSQT element 7.
    E8 = square 60. ==> PSQT element 4.
    E1 = square 4.  ==> PSQT element 60.
*/

// One can also see that, if the SAME PSQT from WHITE's point of view is to
// be used for BLACK, it can be indexed by the square number, without any
// converstion. (Super-impose the square numbers on top of the PSQT with
// BLACK square names.)

// This results in the following converstion table, from aquare number
// to PSQT element, needed for WHITE only::

#[allow(dead_code)]
#[rustfmt::skip]
pub const FLIP: [usize; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
     8,  9, 10, 11, 12, 13, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7,
];
