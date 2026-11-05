use super::board::{self, RAY_EAST, RAY_NORTH, RAY_SOUTH, RAY_WEST};
use super::traits::Piece;

/// A complete set of black and white Rooks
#[derive(Debug, Clone, Copy)]
pub struct RookSet {
    /// Rook bitboards
    bboards: [Rook; 2],
    /// The side of the board
    side: board::Side,
}

impl Piece for RookSet {
    /// Returns a bitboard of all valid Rook Moves
    fn moves(&self) -> u64 {
        let mut moves: u64 = 0;

        for rook in self.bboards.iter() {
            moves |= rook.moves();
        }

        moves
    }

    /// Returns a bitboard of all valid Rook attacks
    fn attacks(&self, blockers: u64) -> u64 {
        let mut moves: u64 = 0;

        for rook in self.bboards.iter() {
            moves |= rook.attacks(blockers);
        }

        moves
    }

    fn piece_square_value(&self) -> i32 {
        unimplemented!()
    }

    /// Returns the board
    fn board(&self) -> u64 {
        let mut board: u64 = 0;
        for rook in self.bboards.iter() {
            board |= rook.board();
        }
        board
    }
}

impl RookSet {
    /// Creates a new RookSet of the given side
    pub fn new(side: board::Side) -> Self {
        let mut bboards: [Rook; 2] = [
            Rook::new(0, side),
            Rook::new(0, side),
        ];

        match side {
            board::Side::White => {
                bboards[0] = Rook::new(board::POSITION_ARRAY[board::A1], side);
                bboards[1] = Rook::new(board::POSITION_ARRAY[board::H1], side);
            }
            board::Side::Black => {
                bboards[0] = Rook::new(board::POSITION_ARRAY[board::A8], side);
                bboards[1] = Rook::new(board::POSITION_ARRAY[board::H8], side);
            }
        }

        RookSet { bboards, side }
    }

    /// Returns the evaluation of the RookSet
    pub fn evaluate(&self) -> i32 {
        let mut score = 0;

        for rook in self.bboards.iter() {
            score += 5 * rook.piece_square_value();
        }

        score
    }
}

/// Rook structure
#[derive(Debug, Clone, Copy)]
pub struct Rook {
    bboard: u64,
    side: board::Side,
}

impl Rook {
    /// Default white rook board
    pub const WHITE_DEFAULT: u64 =
        (board::RANK_1 & board::A_FILE) | (board::RANK_1 & board::H_FILE);
    /// Default black rook board
    pub const BLACK_DEFAULT: u64 =
        (board::RANK_8 & board::A_FILE) | (board::RANK_8 & board::H_FILE);
    pub const ROOK_TABLE: [i32; 64] = [
        0, 0, 0, 5, 5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
        0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10, 10, 10, 5,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];

    /// Creates a new Rook of the given side
    pub fn new(pos: u64, side: board::Side) -> Self {
        todo!()
    }
}

impl Piece for Rook {
    /// Return a bitboard of all valid moves
    fn moves(&self) -> u64 {
        let mut moves: u64 = 0;

        let index = board::bitscan(self.bboard).unwrap() as usize;
        let mut set = RAY_NORTH[index] | RAY_EAST[index] | RAY_SOUTH[index] | RAY_WEST[index];

        while set != 0 {
            moves |= set.trailing_zeros() as u64;
            set &= set - 1;
        }

        moves
    }

    /// Return a bitboard of all vlalid attacks
    fn attacks(&self, blockers: u64) -> u64 {
        todo!()
    }

    fn piece_square_value(&self) -> i32 {
        match self.side {
            board::Side::White => return Rook::ROOK_TABLE[self.bboard.trailing_zeros() as usize],
            board::Side::Black => {
                return Rook::ROOK_TABLE[63 - self.bboard.trailing_zeros() as usize]
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
    fn test_rook() {}
}
