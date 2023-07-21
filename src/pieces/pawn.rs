use std::{fmt, ops::*};
use super::traits::Piece;
use super::board::{self, BoardFlags};

/// A complete set of black and white pawns
///
/// Example
/// ```
/// ```
///
pub struct PawnSet {
    bboards: Vec<Pawn>,
    side: board::Side,
}

impl Piece for PawnSet {

    /// Returns a bitboard of all valid moves in a PawnSet
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();
        let mut set: u64 = 0x00;
        for pawn in self.bboards.iter() {
            moves.extend(pawn.moves());
        }

        moves
    }

    /// Returns a bitboard of all valid attacks in a PawnSet
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self, blockers: u64) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();
        let mut set: u64 = 0x00;
        for pawn in self.bboards.iter() {
            moves.extend(pawn.attacks(blockers));
        }

        moves
    }

    fn piece_square_value(&self) -> i32 {
        unimplemented!()
    }

    fn board(&self) -> u64 {
        let mut board: u64 = 0;
        for pawn in self.bboards {
            board &= pawn.board();
        }
        board
    }
}

impl PawnSet {
    pub fn new(side: board::Side) -> Self {
        let mut bboards: Vec<Pawn> = Vec::new();

        match side {
            board::Side::White => {
                for i in board::A2..board::A3 {
                    bboards.push(Pawn::new(board::POSITION_ARRAY[i], side));
                }
            },
            board::Side::Black => {
                for i in board::A7..board::A8 {
                    bboards.push(Pawn::new(board::POSITION_ARRAY[i], side));
                }
            },
        }

        PawnSet {
            bboards,
            side,
        }
    }

    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        for pawn in self.bboards {
            score += 1 * pawn.piece_square_value();
        }

        score
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
    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.push().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.double_push().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });

        moves
    }

    /// Returns a bitboard of all valid attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self, blockers: u64) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.west_attacks().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.east_attacks().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });

        moves
    }

    fn piece_square_value(&self) -> i32 {
        match self.side {
            board::Side::White => {
                return Pawn::PAWN_TABLE[self.bboard.trailing_zeros() as usize]
            },
            board::Side::Black => {
                return Pawn::PAWN_TABLE[63 - self.bboard.trailing_zeros() as usize]
            }
        }
    }

    fn board(&self) -> u64 {
        self.bboard
    }

}

impl Pawn {
    pub const PAWN_TABLE: [i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,   5, 10, 25, 25, 10,  5,  5,
    0,   0,   0, 20, 20,  0,  0,  0,
    5,  -5,-10,  0,  0,-10, -5,  5,
    5,  10, 10,-20,-20, 10, 10,  5,
    0,   0,   0,   0,   0,   0,   0,   0
    ];

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
        self.push();
        self.push();
        self.bboard
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
