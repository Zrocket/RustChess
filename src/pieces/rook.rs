use std::{fmt, ops};
use super::traits::Piece;
use super::board::{self, RAY_NORTH, RAY_EAST, RAY_SOUTH, RAY_WEST};

/// A complete set of black and white Rooks
///
/// Example
/// ```
/// ```
///
pub struct RookSet {
    bboards: [Rook; 2],
    side: board::Side
}

impl Piece for RookSet {

    /// Returns a bitboard of all valid Rook Moves
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn moves(&self) -> u64 {
        let mut set: u64 = 0x00;
        for rook in self.bboards.iter() {
            set = set & rook.moves();
        }
        set
    }

    /// Returns a bitboard of all valid Rook attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self) -> u64 {
        let mut set: u64 = 0x00;
        for rook in self.bboards.iter() {
            set = set & rook.attacks();
        }
        set
    }
}

/// Rook structure
///
/// Example
/// ```
/// ```
///
pub struct Rook {
    bboard: u64,
    side: board::Side
}

impl Rook {
    pub const WHITE_DEFAULT: u64 = (board::RANK_1 & board::A_FILE) | (board::RANK_1 & board::H_FILE);
    pub const BLACK_DEFAULT: u64 = (board::RANK_8 & board::A_FILE) | (board::RANK_8 & board::H_FILE);

    /// Returns a new Rook bboard of the desiganted side
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn new(&self, side: board::Side) -> Self {
        match side {
            board::Side::White => {
                Self {
                    bboard: Rook::WHITE_DEFAULT,
                    side
                }
            }
            board::Side::Black => {
                Self {
                    bboard: Rook::BLACK_DEFAULT,
                    side
                }
            }
        }
    }
}

impl Piece for Rook {

    /// Return a bitboard of all valid moves
    ///
    /// Example
    /// ```
    /// ```
    ///
    /// # [OPTIONAL: more explanations and code examples in case some specific
    /// # cases have to be explained in details]
    fn moves(&self) -> u64 {
        let index = board::bitscan(self.bboard).unwrap() as usize;

        RAY_NORTH[index] | RAY_EAST[index] | RAY_SOUTH[index] | RAY_WEST[index]
    }

    /// Return a bitboard of all vlalid attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    /// # [OPTIONAL: more explanations and code examples in case some specific
    /// # cases have to be explained in details]
    fn attacks(&self) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rook() {
    }
}
