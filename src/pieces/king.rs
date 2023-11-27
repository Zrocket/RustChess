use super::board;
use super::traits::Piece;
use std::{fmt, ops::*};

///King structure
pub struct King {
    bboard: u64,
    side: board::Side,
}

impl King {
    /// Default white king board
    pub const WHITE_DEFAULT: u64 = 0x8;
    /// Default black king board
    pub const BLACK_DEFAULT: u64 = 0x1000000000000000;
    pub const KING_TABLE: [i32; 64] = [
        -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
        -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40,
        -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20,
        30, 10, 0, 0, 10, 30, 20,
    ];

    /// Creates a new King of the given side
    pub fn new(side: board::Side) -> Self {
        match side {
            board::Side::White => Self {
                bboard: King::WHITE_DEFAULT,
                side,
            },
            board::Side::Black => Self {
                bboard: King::BLACK_DEFAULT,
                side,
            },
        }
    }
}

impl Piece for King {
    /// Returns all moves
    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: board::north_one(self.bboard).trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: board::west_one(self.bboard).trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: board::east_one(self.bboard).trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: board::south_one(self.bboard).trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: board::northeast_one(self.bboard).trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: board::northeast_one(self.bboard).trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: board::southwest_one(self.bboard).trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: board::southeast_one(self.bboard).trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });

        moves
    }

    /// Returns all attacks
    fn attacks(&self, blockers: u64) -> Vec<board::Move> {
        self.moves()
    }

    /// Returns the piece square value
    fn piece_square_value(&self) -> i32 {
        match self.side {
            board::Side::White => return King::KING_TABLE[self.bboard.trailing_zeros() as usize],
            board::Side::Black => {
                return King::KING_TABLE[63 - self.bboard.trailing_zeros() as usize]
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
    use super::*;

    #[test]
    fn test_king() {}
}
