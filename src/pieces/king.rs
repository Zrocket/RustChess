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
    bboard: u64,
    side: board::Side
}

impl King {
    pub const WHITE_DEFAULT: u64 = 0x8;
    pub const BLACK_DEFAULT: u64 = 0x1000000000000000;

    pub fn new(&self, side: board::Side) -> Self {
        match side {
            board::Side::White => {
                Self {
                    bboard: King::WHITE_DEFAULT,
                    side
                }
            }
            board::Side::Black => {
                Self {
                    bboard: King::BLACK_DEFAULT,
                    side
                }
            }
        }
    }
}

impl Piece for King {

    fn moves(&self) -> u64 {
        board::north_one(self.bboard) | board::west_one(self.bboard) | board::east_one(self.bboard) |
        board::south_one(self.bboard) | board::northwest_one(self.bboard) | board::northeast_one(self.bboard) |
        board::southwest_one(self.bboard) | board::southeast_one(self.bboard)
    }

    fn attacks(&self) -> u64 {
        self.moves()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_king() {
    }
}
