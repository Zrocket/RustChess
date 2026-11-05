use super::board;
use super::traits::Piece;

///King structure
#[derive(Debug, Clone, Copy)]
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
    fn moves(&self) -> u64 {

        board::north_one(self.bboard)
            | board::west_one(self.bboard)
            | board::east_one(self.bboard)
            | board::south_one(self.bboard)
            | board::northeast_one(self.bboard)
            | board::northwest_one(self.bboard)
            | board::southwest_one(self.bboard)
            | board::southeast_one(self.bboard)
    }

    /// Returns all attacks
    fn attacks(&self, blockers: u64) -> u64 {
        todo!()
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

    #[test]
    fn test_king() {}
}
