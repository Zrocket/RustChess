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
    bboards: Vec<Knight>,
    side: board::Side,
}

impl Piece for KnightSet {

    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        for knight in self.bboards.iter() {
            moves.extend(knight.moves());
        }

        moves
    }

    fn attacks(&self, blockers: u64) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        for knight in self.bboards.iter() {
            moves.extend(knight.attacks(blockers));
        }

        moves
    }

    fn piece_square_value(&self) -> i32 {
        unimplemented!()
    }

    fn board(&self) -> u64 {
        let mut board: u64 = 0;
        for knight in self.bboards {
            board &= knight.board();
        }
        board
    }
}

impl KnightSet {
    pub fn new(side: board::Side) -> Self {
        let mut bboards: Vec<Knight> = Vec::new();

        match side {
            board::Side::White => {
                bboards.push(Knight::new(board::POSITION_ARRAY[board::B1], side));
                bboards.push(Knight::new(board::POSITION_ARRAY[board::G1], side));
            },
            board::Side::Black => {
                bboards.push(Knight::new(board::POSITION_ARRAY[board::B8], side));
                bboards.push(Knight::new(board::POSITION_ARRAY[board::G8], side));
            },
        }

        KnightSet {
            bboards,
            side,
        }
    }

    pub fn evaluate(&self) -> i32 {
        let mut score = 0;

        for knight in self.bboards {
            score += 3 * knight.piece_square_value();
        }

        score
    }
}

///Knight structure
///
/// Example
/// ```
/// ```
///
pub struct Knight {
    bboard: u64,
    side: board::Side,
}
impl Knight {
    pub const WHITE_DEFAULT: u64 = 0x42;
    pub const BLACK_DEFAULT: u64 = 0x4200000000000000;
    pub const KNIGHT_TABLE: [i32; 64] = [
        -50, -40, -30, -30, -30, -30, -40, -50,
        -40, -20,   0,   5,   5,   0, -20, -40,
        -30,   5,  10,  15,  15,  10,   5, -30,
        -30,   0,  15,  20,  20,  15,   0, -30,
        -30,   5,  15,  20,  20,  15,   5, -30,
        -30,   0,  10,  15,  15,  10,   0, -30,
        -40, -20,   0,   0,   0,   0, -20, -40,
        -50, -40, -30, -30, -30, -30, -40, -50,
    ];

    pub fn new(pos: u64, side: board::Side) -> Self {
        match side {
            board::Side::White => {
                return Knight {
                }
            },
            board::Side::Black => {
                return Knight {
                }
            },
        }
    }

    pub fn north_north_east(&self) -> u64 {
        (self.bboard << 15) & !board::A_FILE
    }

    pub fn north_east_east(&self) -> u64 {
        (self.bboard << 10) & !(board::A_FILE | board::B_FILE)
    }

    pub fn south_east_east(&self) -> u64 {
        (self.bboard >> 6) & !(board::A_FILE | board::B_FILE)
    }

    pub fn south_south_east(&self) -> u64 {
        (self.bboard >> 15) & !board::A_FILE
    }

    pub fn north_north_west(&self) -> u64 {
        (self.bboard << 17) & !board::H_FILE
    }

    pub fn north_west_west(&self) -> u64 {
        (self.bboard << 6) & !(board::G_FILE | board::H_FILE)
    }

    pub fn south_south_west(&self) -> u64 {
        (self.bboard >> 17) & !board::H_FILE
    }
}

impl Piece for Knight {

    fn moves(&self) -> Vec<board::Move> {
        let mut moves: Vec<board::Move> = Vec::new();

        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.north_north_east().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.north_east_east().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.south_east_east().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.south_south_east().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.north_north_west().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.north_west_west().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });
        moves.push(board::Move {
            from: self.bboard.trailing_zeros() as usize,
            to: self.south_south_west().trailing_zeros() as usize,
            flags: board::BoardFlags::empty(),
        });

        moves
    }

    fn attacks(&self, blockers: u64) -> Vec<board::Move> {
        self.moves()
    }

    fn piece_square_value(&self) -> i32 {
        match self.side {
            board::Side::White => {
                return Knight::KNIGHT_TABLE[self.bboard.trailing_zeros() as usize]
            },
            board::Side::Black => {
                return Knight::KNIGHT_TABLE[63 - self.bboard.trailing_zeros() as usize]
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
    fn test_knight() {
    }
}
