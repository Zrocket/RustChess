use std::{fmt, ops::*};
use super::board;
use super::traits::Piece;

///Queen structure
///
/// Example
/// ```
/// ```
///
pub struct Queen {
    piece: u64,
    side: board::Side
}

impl Queen {
    pub const WHITE_DEFAULT: u64 = 0x10;
    pub const BLACK_DEFAULT: u64 = 0x800000000000000;

    pub fn new(&self, side: board::Side) -> Self {
        match side {
            board::Side::White => {
                Self {
                    piece: Queen::WHITE_DEFAULT,
                    side
                }
            }
            board::Side::Black => {
                Self {
                    piece: Queen::BLACK_DEFAULT,
                    side
                }
            }
        }
    }
}

impl Piece for Queen {
    fn bboard(&self) -> u64 {
        self.piece
    }

    fn moves(&self) -> u64 {
        todo!()
    }

    fn attacks(&self) -> u64 {
        todo!()
    }
}
