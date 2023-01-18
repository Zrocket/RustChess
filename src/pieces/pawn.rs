use std::{fmt, ops::*};
use super::traits::Piece;
use super::board;

pub struct PawnSet {
    pieces: [Pawn; 2],
    side: board::Side,
}

impl Piece for PawnSet {
    fn bboard(&self) -> u64 {
        let mut set: u64 = 0x00;
        for pawn in self.pieces.iter() {
            set = set & pawn.bboard();
        }
        set
    }

    fn moves(&self) -> u64 {
        let mut set: u64 = 0x00;
        for pawn in self.pieces.iter() {
            set = set & pawn.moves();
        }
        set
    }

    fn attacks(&self) -> u64 {
        let mut set: u64 = 0x00;
        for pawn in self.pieces.iter() {
            set = set & pawn.attacks();
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
    piece: u64,
    side: board::Side
}

impl Piece for Pawn {
    fn bboard(&self) -> u64 {
        self.piece
    }

    fn moves(&self) -> u64 {
        self.push() | self.double_push()
    }

    fn attacks(&self) -> u64 {
        self.west_attacks() | self.east_attacks()
    }
}

impl Pawn {
    pub fn new(pos: u64, side: board::Side) -> Self {
        Pawn {
            piece: pos,
            side,
        }
    }

    //Bit masks
    pub const WHITE_DEFAULT: u64 = board::RANK_2;
    pub const BLACK_DEFAULT: u64 = board::RANK_7;

    /// [short explanation of what the item does]
    ///
    /// Example
    /// ```
    /// ```
    ///
    /// # [OPTIONAL: more explanations and code examples in case some specific
    /// # cases have to be explained in details]
    pub fn push(&self) -> u64 {
        match self.side {
            board::Side::White => { board::north_one(self.piece) }
            board::Side::Black => { board::south_one(self.piece) }
        }
    }

    /// [short explanation of what the item does]
    ///
    /// Example
    /// ```
    /// ```
    ///
    /// # [OPTIONAL: more explanations and code examples in case some specific
    /// # cases have to be explained in details]
    pub fn double_push(&self) -> u64 {
        let single_push: u64 = self.push();
        match self.side {
            board::Side::White => {
                board::north_one(single_push) & board::RANK_4
            }
            board::Side::Black => {
                board::south_one(single_push) & board::RANK_4
            }
        }
    }

    /// Pawn west attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn west_attacks(&self) -> u64 {
        match self.side {
            board::Side::White => { board::northeast_one(self.piece) }
            board::Side::Black => { board::southwest_one(self.piece) }
        }
    }

    /// Pawn east attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn east_attacks(&self) -> u64 {
        match self.side() {
            board::Side::White => { board::northeast_one(self.piece) }
            board::Side::Black => { board::southeast_one(self.piece) }
        }
    }
}
