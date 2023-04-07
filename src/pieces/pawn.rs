use std::{fmt, ops::*};
use super::traits::Piece;
use super::board;

/// A complete set of black and white pawns
///
/// Example
/// ```
/// ```
///
pub struct PawnSet {
    bboards: [Pawn; 2],
    side: board::Side,
}

impl Piece for PawnSet {

    /// Returns a bitboard of all valid moves in a PawnSet
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn moves(&self) -> u64 {
        let mut set: u64 = 0x00;
        for pawn in self.bboards.iter() {
            set = set & pawn.moves();
        }
        set
    }

    /// Returns a bitboard of all valid attacks in a PawnSet
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self, blockers: u64) -> u64 {
        let mut set: u64 = 0x00;
        for pawn in self.bboards.iter() {
            set = set & pawn.attacks(blockers);
        }
        set
    }
}

/// Pawn structure
///
/// Example
/// ``` ```
///
pub struct Pawn {
    pub bboard: u64,
    pub side: board::Side,
}

impl Piece for Pawn {

    /// Returns a bitboard of all valid moves
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn moves(&self) -> u64 {
        self.push() | self.double_push()
    }

    /// Returns a bitboard of all valid attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self, blockers: u64) -> u64 {
        self.west_attacks() | self.east_attacks()
    }
}

impl Pawn {

    /// Create a new Pawn
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn new(pos: u64, side: board::Side) -> Self {
        Pawn {
            bboard: pos,
            side,
        }
    }

    //Bit masks
    pub const WHITE_DEFAULT: u64 = board::RANK_2;
    pub const BLACK_DEFAULT: u64 = board::RANK_7;

    /// Return a bitboard of a side relevant pawn push
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn push(&self) -> u64 {
        match self.side {
            board::Side::White => { board::north_one(self.bboard) }
            board::Side::Black => { board::south_one(self.bboard) }
        }
    }

    /// Returns a bitboard of a side relevant pawn double push
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn double_push(&self) -> u64 {
        let single_push: u64 = self.push();
        match self.side {
            board::Side::White => {
                board::north_one(single_push)
            }
            board::Side::Black => {
                board::south_one(single_push)
            }
        }
    }

    /// Returns a bitboard of valid west attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn west_attacks(&self) -> u64 {
        match self.side {
            board::Side::White => { board::northeast_one(self.bboard) }
            board::Side::Black => { board::southwest_one(self.bboard) }
        }
    }

    /// Returns a bitboard of valid east attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn east_attacks(&self) -> u64 {
        match self.side {
            board::Side::White => { board::northeast_one(self.bboard) }
            board::Side::Black => { board::southeast_one(self.bboard) }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pawn() {
        let pawn = Pawn::new(0x1, board::Side::White);
        assert_eq!(pawn.push(), 0x100 as u64);
        assert_eq!(pawn.double_push(), 0x10000 as u64);
    }
}
