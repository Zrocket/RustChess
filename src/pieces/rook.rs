use std::{fmt, ops};
use super::traits::Piece;
use super::board::{self, RAY_NORTH, RAY_EAST, RAY_SOUTH, RAY_WEST};

/// A complete set of black and white Rooks
///
/// Example
/// ```
/// ```
///
pub struct RookSet {
    bboards: Vec<Rook>,
    side: board::Side
}

impl Piece for RookSet {

    /// Returns a bitboard of all valid Rook Moves
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        for rook in self.bboards.iter() {
            moves.extend(rook.moves());
        }

        moves
    }

    /// Returns a bitboard of all valid Rook attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self, blockers: u64) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        for rook in self.bboards.iter() {
            moves.extend(rook.attacks(blockers));
        }

        moves
    }

    fn piece_square_value(&self) -> i32 {
        unimplemented!()
    }

    fn board(&self) -> u64 {
        let mut board: u64 = 0;
        for rook in self.bboards {
            board &= rook.board();
        }
        board
    }
}

impl RookSet {
    pub fn new(side: board::Side) -> Self {
        let mut bboards: Vec<Rook> = Vec::new();

        match side {
            board::Side::White => {
                bboards.push(Rook::new(board::POSITION_ARRAY[board::A1], side));
                bboards.push(Rook::new(board::POSITION_ARRAY[board::H1], side));
            },
            board::Side::Black => {
                bboards.push(Rook::new(board::POSITION_ARRAY[board::A8], side));
                bboards.push(Rook::new(board::POSITION_ARRAY[board::H8], side));
            },
        }

        RookSet {
            bboards,
            side,
        }
    }

    pub fn evaluate(&self) -> i32 {
        let mut score = 0;

        for rook in self.bboards {
            score += 5 * rook.piece_square_value();
        }

        score
    }
}

/// Rook structure
///
/// Example
/// ```
/// ```
///
pub struct Rook {
    bboard: u64,
    side: board::Side
}

impl Rook {
    pub const WHITE_DEFAULT: u64 = (board::RANK_1 & board::A_FILE) | (board::RANK_1 & board::H_FILE);
    pub const BLACK_DEFAULT: u64 = (board::RANK_8 & board::A_FILE) | (board::RANK_8 & board::H_FILE);
    pub const ROOK_TABLE: [i32; 64] = [
        0,  0,  0,  5,  5,  0,  0,  0,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        5, 10, 10, 10, 10, 10, 10,  5,
        0,  0,  0,  0,  0,  0,  0,  0
    ];

    /// Returns a new Rook bboard of the desiganted side
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn new(pos: u64, side: board::Side) -> Self {
        todo!()
    }
}

impl Piece for Rook {

    /// Return a bitboard of all valid moves
    ///
    /// Example
    /// ```
    /// ```
    ///
    /// # [OPTIONAL: more explanations and code examples in case some specific
    /// # cases have to be explained in details]
    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        let index = board::bitscan(self.bboard).unwrap() as usize;
        let mut set = RAY_NORTH[index] | RAY_EAST[index] | RAY_SOUTH[index] | RAY_WEST[index];

        while set != 0 {
            let to = set.trailing_zeros();
            moves.push(board::Move {
                from: self.bboard.trailing_zeros() as usize,
                to: to as usize,
                flags: board::BoardFlags::empty(),
            });
            set &= set -1;
        }

        moves
    }

    /// Return a bitboard of all vlalid attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    /// # [OPTIONAL: more explanations and code examples in case some specific
    /// # cases have to be explained in details]
    fn attacks(&self, blockers: u64) -> Vec<board::Move> {
        todo!()
    }

    fn piece_square_value(&self) -> i32 {
        match self.side {
            board::Side::White => {
                return Rook::ROOK_TABLE[self.bboard.trailing_zeros() as usize]
            },
            board::Side::Black => {
                return Rook::ROOK_TABLE[63 - self.bboard.trailing_zeros() as usize]
            }
        }
    }

    fn board(&self) -> u64 {
        self.bboard
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rook() {
    }
}
