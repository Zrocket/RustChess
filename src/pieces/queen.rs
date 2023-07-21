use std::{fmt, ops::*};
use super::board::{self, RAY_WEST, RAY_SOUTH_EAST, RAY_SOUTH, RAY_NORTH, RAY_NORTH_EAST, RAY_NORTH_WEST, RAY_SOUTH_WEST, RAY_EAST};
use super::traits::Piece;

///Queen structure
///
/// Example
/// ```
/// ```
///
pub struct Queen {
    bboard: u64,
    side: board::Side
}

impl Queen {
    pub const WHITE_DEFAULT: u64 = 0x10;
    pub const BLACK_DEFAULT: u64 = 0x800000000000000;
    pub const QUEEN_TABLE: [i32; 64] = [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -5,  0,  5,  5,  5,  5,  0, -5,
        0,  0,  5,  5,  5,  5,  0, -5,
        -10,  5,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ];

    pub fn new(side: board::Side) -> Self {
        match side {
            board::Side::White => {
                Self {
                    bboard: Queen::WHITE_DEFAULT,
                    side
                }
            }
            board::Side::Black => {
                Self {
                    bboard: Queen::BLACK_DEFAULT,
                    side
                }
            }
        }
    }
}

impl Piece for Queen {

    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        let index = self.bboard.trailing_zeros() as usize;
        let mut set = RAY_WEST[index] | RAY_NORTH[index] | RAY_EAST[index] | RAY_SOUTH[index] | RAY_NORTH_EAST[index] | RAY_NORTH_WEST[index] | RAY_SOUTH_EAST[index] | RAY_SOUTH_WEST[index];
        while set != 0 {
            let to = set.trailing_zeros();
            moves.push(board::Move {
                from: self.bboard.trailing_zeros() as usize,
                to: to as usize,
                flags: board::BoardFlags::empty(),
            });
            set &= set -1;
        }

        moves
    }

    fn attacks(&self, pos: u64) -> Vec<board::Move> {
        self.moves()
    }

    fn piece_square_value(&self) -> i32 {
        match self.side {
            board::Side::White => {
                return Queen::QUEEN_TABLE[self.bboard.trailing_zeros() as usize]
            },
            board::Side::Black => {
                return Queen::QUEEN_TABLE[63 - self.bboard.trailing_zeros() as usize]
            }
        }
    }

    fn board(&self) -> u64 {
        self.bboard
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_queen() {
    }
}
