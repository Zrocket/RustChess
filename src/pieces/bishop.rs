use std::{fmt, ops};
use super::board::{self, bitscan, RAY_NORTH_EAST, RAY_NORTH_WEST, RAY_SOUTH_EAST, RAY_SOUTH_WEST};
use super::traits::Piece;

/// [short explanation of what the item does]
///
/// Example
/// ```
/// ```
///
pub struct BishopSet {
    bboards: Vec<Bishop>,
    side: board::Side,
}

impl Piece for BishopSet {

    /// [short explanation of what the item does]
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        for bishop in self.bboards.iter() {
            moves.extend(bishop.moves());
        }

        moves
    }

    /// [short explanation of what the item does]
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self, blockers: u64) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        for bishop in self.bboards {
            moves.extend(bishop.attacks(blockers));
        }

        moves
    }

    fn piece_square_value(&self) -> i32 {
        unimplemented!()
    }

    fn board(&self) -> u64 {
        let mut board: u64 = 0;
        for bishop in self.bboards {
            board &= bishop.board();
        }
        board
    }
}

impl BishopSet {
    pub fn new(side: board::Side) -> BishopSet {
        let mut bboards: Vec<Bishop> = Vec::new();

        match side {
            board::Side::White => {
                bboards.push(Bishop::new(board::POSITION_ARRAY[board::C1], side));
                bboards.push(Bishop::new(board::POSITION_ARRAY[board::F1], side));
            },
            board::Side::Black => {
                bboards.push(Bishop::new(board::POSITION_ARRAY[board::C8], side));
                bboards.push(Bishop::new(board::POSITION_ARRAY[board::F8], side));
            },
        }

        BishopSet {
            bboards,
            side,
        }
    }

    pub fn evaluate(&self) -> i32 {
        let mut score = 0;

        for bishop in self.bboards {
            score += 3 * bishop.piece_square_value();
        }

        score
    }
}

/// Bishop structure
///
/// Example
/// ```
/// ```
///
pub struct Bishop {
    bboard: u64,
    side: board::Side
}

impl Bishop {
    pub const WHITE_DEFAULT: u64 = 0x24;
    pub const BLACK_DEFAULT: u64 = 2400000000000000;
    pub const BISHOP_TABLE: [i32; 64] = [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ];

    /// Return's a new Bishop bboard
    ///
    /// Example
    /// ```
    /// ```
    ///
    pub fn new(pos: u64, side: board::Side) -> Self {
        todo!()
    }
}

impl Piece for Bishop {

    /// Returns a bitboard of valid moves
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        let index = board::bitscan(self.bboard).unwrap() as usize;
        let mut set = RAY_NORTH_EAST[index] | RAY_NORTH_WEST[index] | RAY_SOUTH_EAST[index] | RAY_SOUTH_WEST[index];

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

    /// Returns a bitboard of valid attacks
    ///
    /// Example
    /// ```
    /// ```
    ///
    fn attacks(&self, blockers: u64) -> Vec<board::Move> {
        self.moves()
    }

    fn piece_square_value(&self) -> i32 {
        match self.side {
            board::Side::White => {
                return Bishop::BISHOP_TABLE[self.bboard.trailing_zeros() as usize]
            },
            board::Side::Black => {
                return Bishop::BISHOP_TABLE[63 - self.bboard.trailing_zeros() as usize]
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
    fn test_bishop() {
    }
}
