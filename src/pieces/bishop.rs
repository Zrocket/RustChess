use std::{fmt, ops};
use super::board::{self, bitscan, RAY_NORTH_EAST, RAY_NORTH_WEST, RAY_SOUTH_EAST, RAY_SOUTH_WEST};
use super::traits::Piece;

/// [short explanation of what the item does]
///
/// Example
/// ```
/// ```
///
pub struct BishopSet {
    bboards: [Bishop; 2],
    side: board::Side,
}

impl Piece for BishopSet {

    /// [short explanation of what the item does]
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn moves(&self) -> u64 {
        let mut set: u64 = 0x00;
        for bishop in self.bboards.iter() {
            set = set & bishop.moves();
        }
        set
    }

    /// [short explanation of what the item does]
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self) -> u64 {
        let mut set: u64 = 0x00;
        for bishop in self.bboards.iter() {
            set = set & bishop.attacks();
        }
        set
    }
}

/// Bishop structure
///
/// Example
/// ```
/// ```
///
pub struct Bishop {
    bboard: u64,
    side: board::Side
}

impl Bishop {
    pub const WHITE_DEFAULT: u64 = 0x24;
    pub const BLACK_DEFAULT: u64 = 2400000000000000;

    /// Return's a new Bishop bboard
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn new(&self, side: board::Side) -> Self {
        match side {
            board::Side::White => {
                Self {
                    bboard: Bishop::WHITE_DEFAULT,
                    side
                }
            }
            board::Side::Black => {
                Self {
                    bboard: Bishop::BLACK_DEFAULT,
                    side
                }
            }
        }
    }
}

impl Piece for Bishop {

    /// Returns a bitboard of valid moves
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn moves(&self) -> u64 {
        let index = board::bitscan(self.bboard).unwrap() as usize;
        RAY_NORTH_EAST[index] | RAY_NORTH_WEST[index] | RAY_SOUTH_EAST[index] | RAY_SOUTH_WEST[index]
    }

    /// Returns a bitboard of valid attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self) -> u64 {
        self.moves()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bishop() {
    }
}
