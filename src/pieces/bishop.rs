use std::{fmt, ops};
use super::board;
use super::traits::Piece;

/// [short explanation of what the item does]
///
/// Example
/// ```
/// ```
///
/// # [OPTIONAL: more explanations and code examples in case some specific
/// # cases have to be explained in details]
pub struct BishopSet {
    pieces: [Bishop; 2],
    side: board::Side,
}

impl Piece for BishopSet {
    fn bboard(&self) -> u64 {
        let mut set: u64 = 0x00;
        for bishop in self.pieces.iter() {
            set = set & bishop.bboard();
        }
        set
    }

    fn moves(&self) -> u64 {
        let mut set: u64 = 0x00;
        for bishop in self.pieces.iter() {
            set = set & bishop.moves();
        }
        set
    }

    fn attacks(&self) -> u64 {
        let mut set: u64 = 0x00;
        for bishop in self.pieces.iter() {
            set = set & bishop.attacks();
        }
        set
    }
}

///Bishop structure
///
/// Example
/// ```
/// ```
///
pub struct Bishop {
    piece: u64,
    side: board::Side
}

impl Bishop {
    pub const WHITE_DEFAULT: u64 = 0x24;
    pub const BLACK_DEFAULT: u64 = 2400000000000000;

    pub fn new(&self, side: board::Side) -> Self {
        match side {
            board::Side::White => {
                Self {
                    piece: Bishop::WHITE_DEFAULT,
                    side
                }
            }
            board::Side::Black => {
                Self {
                    piece: Bishop::BLACK_DEFAULT,
                    side
                }
            }
        }
    }
}

impl Piece for Bishop {
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
