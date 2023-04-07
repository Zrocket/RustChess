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

    pub fn new(&self, side: board::Side) -> Self {
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

    fn moves(&self) -> u64 {
        let index = board::bitscan(self.bboard).unwrap() as usize;

         RAY_WEST[index] | RAY_NORTH[index] | RAY_EAST[index] | RAY_SOUTH[index] | RAY_NORTH_EAST[index] | RAY_NORTH_WEST[index] | RAY_SOUTH_EAST[index] | RAY_SOUTH_WEST[index]
    }

    fn attacks(&self) -> u64 {
        self.moves()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_queen() {
    }
}
