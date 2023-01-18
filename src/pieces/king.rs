use std::{fmt, ops::*};
use super::board;
use super::traits::Piece;

///King structure
///
/// Example
/// ```
/// ```
///
pub struct King {
    piece: u64,
    side: board::Side
}

impl King {
    pub const WHITE_DEFAULT: u64 = 0x8;
    pub const BLACK_DEFAULT: u64 = 0x1000000000000000;

    pub fn new(&self, side: board::Side) -> Self {
        match side {
            board::Side::White => {
                Self {
                    piece: King::WHITE_DEFAULT,
                    side
                }
            }
            board::Side::Black => {
                Self {
                    piece: King::BLACK_DEFAULT,
                    side
                }
            }
        }
    }
}

impl Piece for King {
    fn bboard(&self) -> u64 {
        self.piece
    }

    fn moves(&self) -> u64 {
        board::north_one(self.piece) | board::west_one(self.piece) | board::east_one(self.piece) |
        board::south_one(self.piece) | board::northwest_one(self.piece) | board::northeast_one(self.piece) |
        board::southwest_one(self.piece) | board::southeast_one(self.piece)
    }

    fn attacks(&self) -> u64 {
        board::north_one(self.piece) | board::west_one(self.piece) | board::east_one(self.piece) |
        board::south_one(self.piece) | board::northwest_one(self.piece) | board::northeast_one(self.piece) |
        board::southwest_one(self.piece) | board::southeast_one(self.piece)
    }
}
