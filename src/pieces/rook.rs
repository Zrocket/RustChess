use std::{fmt, ops};
use super::traits::Piece;
use super::board;

/// [short explanation of what the item does]
///
/// Example
/// ```
/// ```
///
/// # [OPTIONAL: more explanations and code examples in case some specific
/// # cases have to be explained in details]
pub struct RookSet {
    pieces: [Rook; 2],
    side: board::Side
}

impl Piece for RookSet {
    fn bboard(&self) -> u64 {
        let mut set: u64 = 0x00;
        for rook in self.pieces.iter() {
            set = set & rook.bboard();
        }
        set
    }

    fn moves(&self) -> u64 {
        let mut set: u64 = 0x00;
        for rook in self.pieces.iter() {
            set = set & rook.moves();
        }
        set
    }

    fn attacks(&self) -> u64 {
        let mut set: u64 = 0x00;
        for rook in self.pieces.iter() {
            set = set & rook.attacks();
        }
        set
    }
}

///Rook structure
/// [short explanation of what the item does]
///
/// Example
/// ```
/// ```
///
pub struct Rook {
    piece: u64,
    side: board::Side
}

impl Rook {
    pub const WHITE_DEFAULT: u64 = (board::RANK_1 & board::A_FILE) | (board::RANK_1 & board::H_FILE);
    pub const BLACK_DEFAULT: u64 = (board::RANK_8 & board::A_FILE) | (board::RANK_8 & board::H_FILE);

    pub fn new(&self, side: board::Side) -> Self {
        match side {
            board::Side::White => {
                Self {
                    piece: Rook::WHITE_DEFAULT,
                    side
                }
            }
            board::Side::Black => {
                Self {
                    piece: Rook::BLACK_DEFAULT,
                    side
                }
            }
        }
    }
}

impl Piece for Rook {
    fn bboard(&self) -> &u64 {
        &self.piece
    }

    fn moves(&self) -> &u64 {
        todo!()
    }

    fn attacks(&self) -> &u64 {
        todo!()
    }
}
