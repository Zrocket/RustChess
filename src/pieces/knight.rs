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
pub struct KnightSet {
    pieces: [Knight; 2],
    side: board::Side,
}

impl Piece for KnightSet {
    fn bboard(&self) -> u64 {
        let mut set: u64 = 0x00;
        for knight in self.pieces.iter() {
            set = set & knight.bboard();
        }
        set
    }

    fn moves(&self) -> u64 {
        let mut set: u64 = 0x00;
        for knight in self.pieces.iter() {
            set = set & knight.moves();
        }
        set
    }

    fn attacks(&self) -> u64 {
        let mut set: u64 = 0x00;
        for knight in self.pieces.iter() {
            set = set & knight.attacks();
        }
        set
    }
}

///Knight structure
///
/// Example
/// ```
/// ```
///
pub struct Knight {
    piece: u64,
    side: board::Side,
}
impl Knight {
    pub const WHITE_DEFAULT: u64 = 0x42;
    pub const BLACK_DEFAULT: u64 = 0x4200000000000000;

    fn north_north_east(&self) -> u64 {
        (self.piece << 15) & !board::A_FILE
    }
    fn north_east_east(&self) -> u64 {
        (self.piece << 10) & !(board::A_FILE | board::B_FILE)
    }
    fn south_east_east(&self) -> u64 {
        (self.piece >> 6) & !(board::A_FILE | board::B_FILE)
    }
    fn south_south_east(&self) -> u64 {
        (self.piece >> 15) & !board::A_FILE
    }
    fn north_north_west(&self) -> u64 {
        (self.piece << 17) & !board::H_FILE
    }
    fn north_west_west(&self) -> u64 {
        (self.piece << 6) & !(board::G_FILE | board::H_FILE)
    }
    fn south_south_west(&self) -> u64 {
        (self.piece >> 17) & !board::H_FILE
    }
    fn fill(&self) -> u64 {
        Knight::attacks(self) | self.piece
    }
}

impl Piece for Knight {
    fn bboard(&self) -> u64 {
        self.piece
    }

    fn moves(&self) -> u64 {
        Knight::north_north_east(self) | Knight::north_east_east(self) | Knight::south_east_east(self) |
        Knight::south_south_east(self) | Knight::north_north_west(self) | Knight::north_west_west(self) | 
        Knight::south_south_west(self)
    }

    fn attacks(&self) -> u64 {
        Knight::north_north_east(self) | Knight::north_east_east(self) | Knight::south_east_east(self) |
        Knight::south_south_east(self) | Knight::north_north_west(self) | Knight::north_west_west(self) | 
        Knight::south_south_west(self)
    }
}
