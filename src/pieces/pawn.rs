use super::board;
use super::traits::Piece;

/// A complete set of black and white pawns
#[derive(Debug, Clone, Copy)]
pub struct PawnSet {
    bboards: [Pawn; 8],
    side: board::Side,
}

impl Piece for PawnSet {
    /// Returns a bitboard of all valid moves in a PawnSet
    fn moves(&self) -> u64 {
        let mut moves: u64 = 0;

        for pawn in self.bboards.iter() {
            moves |= pawn.moves();
        }

        moves
    }

    /// Returns a bitboard of all valid attacks in a PawnSet
    fn attacks(&self, blockers: u64) -> u64 {
        let mut moves: u64 = 0;

        for pawn in self.bboards.iter() {
            moves |= pawn.attacks(blockers);
        }

        moves
    }

    fn piece_square_value(&self) -> i32 {
        unimplemented!()
    }

    /// Returns the board
    fn board(&self) -> u64 {
        let mut board: u64 = 0;
        for pawn in self.bboards.iter() {
            board |= pawn.board();
        }
        board
    }
}

impl PawnSet {
    /// Creates a new PawnSet of the given side
    pub fn new(side: board::Side) -> Self {
        let mut bboards: [Pawn; 8] = [
            Pawn::new(0, side),
            Pawn::new(0, side),
            Pawn::new(0, side),
            Pawn::new(0, side),
            Pawn::new(0, side),
            Pawn::new(0, side),
            Pawn::new(0, side),
            Pawn::new(0, side),
        ];

        match side {
            board::Side::White => {
                for i in board::A2..board::A3 {
                    bboards[i - board::A2] = Pawn::new(board::POSITION_ARRAY[i], side);
                }
            }
            board::Side::Black => {
                for i in board::A7..board::A8 {
                    bboards[i - board::A7] = Pawn::new(board::POSITION_ARRAY[i], side);
                }
            }
        }

        PawnSet { bboards, side }
    }

    /// Evaluates the PawnSet
    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        for pawn in self.bboards.iter() {
            score += 1 * pawn.piece_square_value();
        }

        score
    }
}

/// Pawn structure
#[derive(Debug, Clone, Copy)]
pub struct Pawn {
    pub bboard: u64,
    pub side: board::Side,
}

impl Piece for Pawn {
    /// Returns a bitboard of all valid moves
    fn moves(&self) -> u64 {
        self.push()
            | self.double_push()
    }

    /// Returns a bitboard of all valid attacks
    fn attacks(&self, blockers: u64) -> u64 {
        self.west_attacks()
            | self.east_attacks()
    }

    fn piece_square_value(&self) -> i32 {
        match self.side {
            board::Side::White => return Pawn::PAWN_TABLE[self.bboard.trailing_zeros() as usize],
            board::Side::Black => {
                return Pawn::PAWN_TABLE[63 - self.bboard.trailing_zeros() as usize]
            }
        }
    }

    /// Returns the board
    fn board(&self) -> u64 {
        self.bboard
    }
}

impl Pawn {
    pub const PAWN_TABLE: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5,
        5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10,
        -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    /// Create a new Pawn of the given side
    pub fn new(pos: u64, side: board::Side) -> Self {
        Pawn { bboard: pos, side }
    }

    /// Default board for white pawns
    pub const WHITE_DEFAULT: u64 = board::RANK_2;
    /// Default board for black pawns
    pub const BLACK_DEFAULT: u64 = board::RANK_7;

    /// Return a bitboard of a side relevant pawn push
    pub fn push(&self) -> u64 {
        match self.side {
            board::Side::White => board::north_one(self.bboard),
            board::Side::Black => board::south_one(self.bboard),
        }
    }

    /// Returns a bitboard of a side relevant pawn double push
    pub fn double_push(&self) -> u64 {
        match self.side {
            board::Side::White => board::north_one(self.push()),
            board::Side::Black => board::south_one(self.push()),
        }
    }

    /// Returns a bitboard of valid west attacks
    pub fn west_attacks(&self) -> u64 {
        match self.side {
            board::Side::White => board::northwest_one(self.bboard),
            board::Side::Black => board::southwest_one(self.bboard),
        }
    }

    /// Returns a bitboard of valid east attacks
    pub fn east_attacks(&self) -> u64 {
        match self.side {
            board::Side::White => board::northeast_one(self.bboard),
            board::Side::Black => board::southeast_one(self.bboard),
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
