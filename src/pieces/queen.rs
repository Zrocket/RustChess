use super::board::{
    self, RAY_EAST, RAY_NORTH, RAY_NORTH_EAST, RAY_NORTH_WEST, RAY_SOUTH, RAY_SOUTH_EAST,
    RAY_SOUTH_WEST, RAY_WEST,
};
use super::traits::Piece;

///Queen structure
///
/// Example
/// ```
/// ```
///
#[derive(Debug, Clone, Copy)]
pub struct Queen {
    bboard: u64,
    side: board::Side,
}

impl Queen {
    /// Default white queen board
    pub const WHITE_DEFAULT: u64 = 0x10;
    /// Default black queen board
    pub const BLACK_DEFAULT: u64 = 0x800000000000000;
    pub const QUEEN_TABLE: [i32; 64] = [
        -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0,
        -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0,
        5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
    ];

    /// Creates a new Queen of the given side
    pub fn new(side: board::Side) -> Self {
        match side {
            board::Side::White => Self {
                bboard: Queen::WHITE_DEFAULT,
                side,
            },
            board::Side::Black => Self {
                bboard: Queen::BLACK_DEFAULT,
                side,
            },
        }
    }
}

impl Piece for Queen {
    /// Returns all moves
    fn moves(&self) -> u64 {
        let index = self.bboard.trailing_zeros() as usize;

        RAY_NORTH[index]
            | RAY_EAST[index]
            | RAY_SOUTH[index]
            | RAY_WEST[index]
            | RAY_NORTH_EAST[index]
            | RAY_NORTH_WEST[index]
            | RAY_SOUTH_EAST[index]
            | RAY_SOUTH_WEST[index]
    }

    /// Returns all attacks
    fn attacks(&self, pos: u64) -> u64 {
        self.moves()
    }

    fn piece_square_value(&self) -> i32 {
        match self.side {
            board::Side::White => return Queen::QUEEN_TABLE[self.bboard.trailing_zeros() as usize],
            board::Side::Black => {
                return Queen::QUEEN_TABLE[63 - self.bboard.trailing_zeros() as usize]
            }
        }
    }

    /// Returns the board
    fn board(&self) -> u64 {
        self.bboard
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_queen() {}
}
