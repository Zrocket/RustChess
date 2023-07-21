use std::collections::HashMap;
use bitflags::bitflags;
use super::{pawn::PawnSet, knight::KnightSet, bishop::BishopSet, queen::Queen, king::King, traits::Piece, rook::RookSet};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct BoardFlags: u32 {
        const capture       = 0b000000000001;
        const passant       = 0b000000000010;
        const castle_king   = 0b000000000100;
        const castle_queen  = 0b000000001000;
        const pawn          = 0b000000010000;
        const knight        = 0b000000100000;
        const bishop        = 0b000001000000;
        const rook          = 0b000010000000;
        const queen         = 0b000100000000;
        const white         = 0b001000000000;
        const black         = 0b010000000000;
        const promotion     = 0b100000000000;
    }
}

pub struct Move {
    pub from: usize,
    pub to: usize,
    pub flags: BoardFlags,
}

pub struct Board {
    white: Player,
    black: Player,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            white: Player::new(Side::White),
            black: Player::new(Side::Black),
        }
    }
}

impl Board {
    pub fn is_game_over(&self) -> bool {
        if self.white.moves().len() == 0 || self.black.moves().len() == 0 {
            return true;
        }

        false
    }

    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        0
    }

    pub fn moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn count_pieces(&self, side: Side) {
        match side {
            Side::White => (),
            Side::Black => (),
        }
    }
}

pub struct Player {
    pawns: PawnSet,
    knights: KnightSet,
    bishops: BishopSet,
    rooks: RookSet,
    queen: Queen,
    king: King,
}

impl Piece for Player {
    fn moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        moves.extend(self.pawns.moves());
        moves.extend(self.knights.moves());
        moves.extend(self.bishops.moves());
        moves.extend(self.rooks.moves());
        moves.extend(self.queen.moves());
        moves.extend(self.king.moves());

        moves
    }

    fn attacks(&self, blockers: u64) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        moves.extend(self.pawns.attacks(blockers));
        moves.extend(self.knights.attacks(blockers));
        moves.extend(self.bishops.attacks(blockers));
        moves.extend(self.rooks.attacks(blockers));
        moves.extend(self.queen.attacks(blockers));
        moves.extend(self.king.attacks(blockers));

        moves
    }

    fn piece_square_value(&self) -> i32 {
        unimplemented!()
    }

    fn board(&self) -> u64 {
        let mut board: u64 = 0;
        board &= self.pawns.board();
        board &= self.knights.board();
        board &= self.bishops.board();
        board &= self.rooks.board();
        board &= self.queen.board();
        board &= self.king.board();
        board
    }
}

impl Player {
    pub fn new(side: Side) -> Self {
        Player {
            pawns: PawnSet::new(side),
            knights: KnightSet::new(side),
            bishops: BishopSet::new(side),
            rooks: RookSet::new(side),
            queen: Queen::new(side),
            king: King::new(side),
        }
    }

    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        score += self.pawns.evaluate();
        score += self.knights.evaluate();
        score += self.bishops.evaluate();
        score += self.rooks.evaluate();
        score += 9 * self.queen.piece_square_value();
        score += self.king.piece_square_value();
        score
    }
}

/// Labels for every Square on the board.
///
/// Example
/// ```
/// ```
///
pub const A1: usize = 0;
pub const B1: usize = 1;
pub const C1: usize = 2;
pub const D1: usize = 3;
pub const E1: usize = 4;
pub const F1: usize = 5;
pub const G1: usize = 6;
pub const H1: usize = 7;

pub const A2: usize = 8;
pub const B2: usize = 9;
pub const C2: usize = 10;
pub const D2: usize = 11;
pub const E2: usize = 12;
pub const F2: usize = 13;
pub const G2: usize = 14;
pub const H2: usize = 15;

pub const A3: usize = 16;
pub const B3: usize = 17;
pub const C3: usize = 18;
pub const D3: usize = 19;
pub const E3: usize = 20;
pub const F3: usize = 21;
pub const G3: usize = 22;
pub const H3: usize = 23;

pub const A4: usize = 24;
pub const B4: usize = 25;
pub const C4: usize = 26;
pub const D4: usize = 27;
pub const E4: usize = 28;
pub const F4: usize = 29;
pub const G4: usize = 30;
pub const H4: usize = 31;

pub const A5: usize = 32;
pub const B5: usize = 33;
pub const C5: usize = 34;
pub const D5: usize = 35;
pub const E5: usize = 36;
pub const F5: usize = 37;
pub const G5: usize = 38;
pub const H5: usize = 39;

pub const A6: usize = 40;
pub const B6: usize = 41;
pub const C6: usize = 42;
pub const D6: usize = 43;
pub const E6: usize = 44;
pub const F6: usize = 45;
pub const G6: usize = 46;
pub const H6: usize = 47;

pub const A7: usize = 48;
pub const B7: usize = 49;
pub const C7: usize = 50;
pub const D7: usize = 51;
pub const E7: usize = 52;
pub const F7: usize = 53;
pub const G7: usize = 54;
pub const H7: usize = 55;

pub const A8: usize = 56;
pub const B8: usize = 57;
pub const C8: usize = 58;
pub const D8: usize = 59;
pub const E8: usize = 60;
pub const F8: usize = 61;
pub const G8: usize = 62;
pub const H8: usize = 63;

pub const POSITION_ARRAY: [u64; 64] = [
    0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80,
    0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000,
    0x10000, 0x20000, 0x40000, 0x80000, 0x100000, 0x200000, 0x400000, 0x800000,
    0x1000000, 0x2000000, 0x4000000, 0x8000000, 0x10000000, 0x20000000, 0x40000000, 0x80000000,
    0x100000000, 0x200000000, 0x400000000, 0x800000000, 0x1000000000, 0x2000000000, 0x4000000000, 0x8000000000,
    0x10000000000, 0x20000000000, 0x40000000000, 0x80000000000, 0x100000000000, 0x200000000000, 0x400000000000, 0x800000000000,
    0x1000000000000, 0x2000000000000, 0x4000000000000, 0x8000000000000, 0x10000000000000, 0x20000000000000, 0x40000000000000, 0x80000000000000,
    0x100000000000000, 0x200000000000000, 0x400000000000000, 0x800000000000000, 0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000,
];

pub const RAY_NORTH: [u64; 64] = [
    0x101010101010101, 0x202020202020202, 0x404040404040404, 0x808080808080808, 0x1010101010101010, 0x2020202020202020, 0x4040404040404040, 0x8080808080808080,
    0x101010101010100, 0x202020202020200, 0x404040404040400, 0x808080808080800, 0x1010101010101000, 0x2020202020202000, 0x4040404040404000, 0x8080808080808000,
    0x101010101010000, 0x202020202020000, 0x404040404040000, 0x808080808080000, 0x1010101010100000, 0x2020202020200000, 0x4040404040400000, 0x8080808080800000,
    0x101010101000000, 0x202020202000000, 0x404040404000000, 0x808080808000000, 0x1010101010000000, 0x2020202020000000, 0x4040404040000000, 0x8080808080000000,
    0x101010100000000, 0x202020200000000, 0x404040400000000, 0x808080800000000, 0x1010101000000000, 0x2020202000000000, 0x4040404000000000, 0x8080808000000000,
    0x101010000000000, 0x202020000000000, 0x404040000000000, 0x808080000000000, 0x1010100000000000, 0x2020200000000000, 0x4040400000000000, 0x8080800000000000,
    0x101000000000000, 0x202000000000000, 0x404000000000000, 0x808000000000000, 0x1010000000000000, 0x2020000000000000, 0x4040000000000000, 0x8080000000000000,
    0x100000000000000, 0x200000000000000, 0x400000000000000, 0x800000000000000, 0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000
];

pub const RAY_EAST: [u64; 64] = [
    0xff, 0xfe, 0xfc, 0xf8, 0xf0, 0xe0, 0xc0, 0x80,
    0xff00, 0xfe00, 0xfc00, 0xf800, 0xf000, 0xe000, 0xc000, 0x8000,
    0xff0000, 0xfe0000, 0xfc0000, 0xf80000, 0xf00000, 0xe00000, 0xc00000, 0x800000,
    0xff000000, 0xfe000000, 0xfc000000, 0xf8000000, 0xf0000000, 0xe0000000, 0xc0000000, 0x80000000,
    0xff00000000, 0xfe00000000, 0xfc00000000, 0xf800000000, 0xf000000000, 0xe000000000, 0xc000000000, 0x8000000000,
    0xff0000000000, 0xfe0000000000, 0xfc0000000000, 0xf80000000000, 0xf00000000000, 0xe00000000000, 0xc00000000000, 0x800000000000,
    0xff000000000000, 0xfe000000000000, 0xfc000000000000, 0xf8000000000000, 0xf0000000000000, 0xe0000000000000, 0xc0000000000000, 0x80000000000000,
    0xff00000000000000, 0xfe00000000000000, 0xfc00000000000000, 0xf800000000000000, 0xf000000000000000, 0xe000000000000000, 0xc000000000000000, 0x8000000000000000
];

pub const RAY_SOUTH: [u64; 64] = [
    0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80,
    0x101, 0x202, 0x404, 0x808, 0x1010, 0x2020, 0x4040, 0x8080,
    0x10101, 0x20202, 0x40404, 0x80808, 0x101010, 0x202020, 0x404040, 0x808080,
    0x1010101, 0x2020202, 0x4040404, 0x8080808, 0x10101010, 0x20202020, 0x40404040, 0x80808080,
    0x101010101, 0x202020202, 0x404040404, 0x808080808, 0x1010101010, 0x2020202020, 0x4040404040, 0x8080808080,
    0x10101010101, 0x20202020202, 0x40404040404, 0x80808080808, 0x101010101010, 0x202020202020, 0x404040404040, 0x808080808080,
    0x1010101010101, 0x2020202020202, 0x4040404040404, 0x8080808080808, 0x10101010101010, 0x20202020202020, 0x40404040404040, 0x80808080808080,
    0x101010101010101, 0x202020202020202, 0x404040404040404, 0x808080808080808, 0x1010101010101010, 0x2020202020202020, 0x4040404040404040, 0x8080808080808080,
];

pub const RAY_WEST: [u64; 64] = [
    0x1, 0x3, 0x7, 0xf, 0x1f, 0x3f, 0x7f, 0xff,
    0x100, 0x300, 0x700, 0xf00, 0x1f00, 0x3f00, 0x7f00, 0xff00,
    0x10000, 0x30000, 0x70000, 0xf0000, 0x1f0000, 0x3f0000, 0x7f0000, 0xff0000,
    0x1000000, 0x3000000, 0x7000000, 0xf000000, 0x1f000000, 0x3f000000, 0x7f000000, 0xff000000,
    0x100000000, 0x300000000, 0x700000000, 0xf00000000, 0x1f00000000, 0x3f00000000, 0x7f00000000, 0xff00000000,
    0x10000000000, 0x30000000000, 0x70000000000, 0xf0000000000, 0x1f0000000000, 0x3f0000000000, 0x7f0000000000, 0xff0000000000,
    0x1000000000000, 0x3000000000000, 0x7000000000000, 0xf000000000000, 0x1f000000000000, 0x3f000000000000, 0x7f000000000000, 0xff000000000000,
    0x100000000000000, 0x300000000000000, 0x700000000000000, 0xf00000000000000, 0x1f00000000000000, 0x3f00000000000000, 0x7f00000000000000, 0xff00000000000000
];

pub const RAY_NORTH_WEST: [u64; 64] = [
    0x1, 0x102, 0x10204, 0x1020408, 0x102040810, 0x10204081020, 0x1020408102040, 0x102040810204080,
    0x100, 0x10200, 0x1020400, 0x102040800, 0x10204081000, 0x1020408102000, 0x102040810204000, 0x204081020408000,
    0x10000, 0x1020000, 0x102040000, 0x10204080000, 0x1020408100000, 0x102040810200000, 0x204081020400000, 0x408102040800000,
    0x1000000, 0x102000000, 0x10204000000, 0x1020408000000, 0x102040810000000, 0x204081020000000, 0x408102040000000, 0x810204080000000,
    0x100000000, 0x10200000000, 0x1020400000000, 0x102040800000000, 0x204081000000000, 0x408102000000000, 0x810204000000000, 0x1020408000000000,
    0x10000000000, 0x1020000000000, 0x102040000000000, 0x204080000000000, 0x408100000000000, 0x810200000000000, 0x1020400000000000, 0x2040800000000000,
    0x1000000000000, 0x102000000000000, 0x204000000000000, 0x408000000000000, 0x810000000000000, 0x1020000000000000, 0x2040000000000000, 0x4080000000000000,
    0x100000000000000, 0x200000000000000, 0x400000000000000, 0x800000000000000, 0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000
];

pub const RAY_NORTH_EAST: [u64; 64] = [
    0x8040201008040201, 0x80402010080402, 0x804020100804, 0x8040201008, 0x80402010, 0x804020, 0x8040, 0x80,
    0x4020100804020100, 0x8040201008040200, 0x80402010080400, 0x804020100800, 0x8040201000, 0x80402000, 0x804000, 0x8000,
    0x2010080402010000, 0x4020100804020000, 0x8040201008040000, 0x80402010080000, 0x804020100000, 0x8040200000, 0x80400000, 0x800000,
    0x1008040201000000, 0x2010080402000000, 0x4020100804000000, 0x8040201008000000, 0x80402010000000, 0x804020000000, 0x8040000000, 0x80000000,
    0x804020100000000, 0x1008040200000000, 0x2010080400000000, 0x4020100800000000, 0x8040201000000000, 0x80402000000000, 0x804000000000, 0x8000000000,
    0x402010000000000, 0x804020000000000, 0x1008040000000000, 0x2010080000000000, 0x4020100000000000, 0x8040200000000000, 0x80400000000000, 0x800000000000,
    0x201000000000000, 0x402000000000000, 0x804000000000000, 0x1008000000000000, 0x2010000000000000, 0x4020000000000000, 0x8040000000000000, 0x80000000000000,
    0x100000000000000, 0x200000000000000, 0x400000000000000, 0x800000000000000, 0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000
];

pub const RAY_SOUTH_WEST: [u64; 64] = [
    0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80,
    0x100, 0x201, 0x402, 0x804, 0x1008, 0x2010, 0x4020, 0x8040,
    0x10000, 0x20100, 0x40201, 0x80402, 0x100804, 0x201008, 0x402010, 0x804020,
    0x1000000, 0x2010000, 0x4020100, 0x8040201, 0x10080402, 0x20100804, 0x40201008, 0x80402010,
    0x100000000, 0x201000000, 0x402010000, 0x804020100, 0x1008040201, 0x2010080402, 0x4020100804, 0x8040201008,
    0x10000000000, 0x20100000000, 0x40201000000, 0x80402010000, 0x100804020100, 0x201008040201, 0x402010080402, 0x804020100804,
    0x1000000000000, 0x2010000000000, 0x4020100000000, 0x8040201000000, 0x10080402010000, 0x20100804020100, 0x40201008040201, 0x80402010080402,
    0x100000000000000, 0x201000000000000, 0x402010000000000, 0x804020100000000, 0x1008040201000000, 0x2010080402010000, 0x4020100804020100, 0x8040201008040201,
];

pub const RAY_SOUTH_EAST: [u64; 64] = [
    0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80,
    0x102, 0x204, 0x408, 0x810, 0x1020, 0x2040, 0x4080, 0x8000,
    0x10204, 0x20408, 0x40810, 0x81020, 0x102040, 0x204080, 0x408000, 0x800000,
    0x1020408, 0x2040810, 0x4081020, 0x8102040, 0x10204080, 0x20408000, 0x40800000, 0x80000000,
    0x102040810, 0x204081020, 0x408102040, 0x810204080, 0x1020408000, 0x2040800000, 0x4080000000, 0x8000000000,
    0x10204081020, 0x20408102040, 0x40810204080, 0x81020408000, 0x102040800000, 0x204080000000, 0x408000000000, 0x800000000000,
    0x1020408102040, 0x2040810204080, 0x4081020408000, 0x8102040800000, 0x10204080000000, 0x20408000000000, 0x40800000000000, 0x80000000000000,
    0x102040810204080, 0x204081020408000, 0x408102040800000, 0x810204080000000, 0x1020408000000000, 0x2040800000000000, 0x4080000000000000, 0x8000000000000000,
];

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Side {
    White,
    Black
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Ray {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub const EMPTY: u64                = 0x0000000000000000;
pub const FULL: u64                 = 0xFFFFFFFFFFFFFFFF;
// Columns
pub const H_FILE: u64               = 0x8080808080808080;
pub const G_FILE: u64               = 0x4040404040404040;
pub const F_FILE: u64               = 0x2020202020202020;
pub const E_FILE: u64               = 0x1010101010101010;
pub const D_FILE: u64               = 0x808080808080808;
pub const C_FILE: u64               = 0x404040404040404;
pub const B_FILE: u64               = 0x202020202020202;
pub const A_FILE: u64               = 0x101010101010101;
// Rows
pub const RANK_1: u64               = 0x00000000000000FF;
pub const RANK_2: u64               = 0x000000000000FF00;
pub const RANK_3: u64               = 0x0000000000FF0000;
pub const RANK_4: u64               = 0x00000000FF000000;
pub const RANK_5: u64               = 0x000000FF00000000;
pub const RANK_6: u64               = 0x0000FF0000000000;
pub const RANK_7: u64               = 0x00FF000000000000;
pub const RANK_8: u64               = 0xFF00000000000000;
// Diagonals
pub const A1_H8_DIAGONAL: u64       = 0x8040201008040201;
pub const H1_A8_ANTIDIAGONAL: u64   = 0x0102040810204080;
// Colors
pub const LIGHT_SQUARES: u64        = 0x55AA55AA55AA55AA;
pub const DARK_SQUARES: u64         = 0xAA55AA55AA55AA55;

pub fn bitscan(board: u64) -> Option<u64> {
    if board == 0 {
        return None;
    }

    Some(board.trailing_zeros() as u64)
}

pub fn get_rows_and_col(board: u64) -> u64 {
    todo!()
}

pub fn get_diagonals(board: u64) -> u64 {
    todo!()
}

/// Shift board northwest one square
///
/// Example
/// ```
/// ```
///
pub fn northwest_one(board: u64) -> u64 {
    (board << 7) & !H_FILE
}

/// Shift board north one square
///
/// Example
/// ```
/// ```
///
pub fn north_one(board: u64) -> u64 {
    board << 8
}

/// Shift board northeast one square
///
/// Example
/// ```
/// ```
///
pub fn northeast_one(board: u64) -> u64 {
    (board << 9) & !A_FILE
}

/// Shift board west one square
///
/// Example
/// ```
/// ```
///
pub fn west_one(board: u64) -> u64 {
    (board >> 1) & !H_FILE
}

/// Shift board east one square
///
/// Example
/// ```
/// ```
///
pub fn east_one(board: u64) -> u64 {
    (board << 1) & !A_FILE
}

/// Shift board southwest one square
///
/// Example
/// ```
/// ```
///
pub fn southwest_one(board: u64) -> u64 {
    (board >> 9) & !H_FILE
}

/// Shift board south one square
///
/// Example
/// ```
/// ```
///
pub fn south_one(board: u64) -> u64 {
    board >> 8
}

/// Shift board southeast one square
///
/// Example
/// ```
/// ```
///
pub fn southeast_one(board: u64) -> u64 {
    (board >> 7) & !A_FILE
}

/// Rotate board left
///
/// Example
/// ```
/// ```
///
pub fn rotate_left(board: u64, s: i32) -> u64 {
    (board << s) | board >> (64 - s)
}

/// Rotate board right
///
/// Example
/// ```
/// ```
///
pub fn rotate_right(board: u64, s: i32) -> u64 {
    (board >> s) | board << (64 - s)
}

pub fn get_ray(mut board: u64, direction: Ray) -> u64 {
    let mut temp = board.clone();

    match direction {
        Ray::North => {
            while temp & RANK_8 == 0 {
                temp = north_one(temp);
                board |= temp;
            }
            board
        },
        Ray::NorthEast => {
            while (temp & RANK_8 == 0) && (temp & H_FILE == 0) {
                temp = north_one(temp);
                temp = east_one(temp);
                board |= temp;
            }
            board
        },
        Ray::East => {
            while temp & H_FILE == 0 {
                temp = east_one(temp);
                board |= temp;
            }
            board
        },
        Ray::SouthEast => {
            while (temp & RANK_1 == 0) && (temp & H_FILE == 0) {
                temp = south_one(temp);
                temp = east_one(temp);
                board |= temp;
            }
            board
        },
        Ray::South => {
            while temp & RANK_1 == 0 {
                temp = south_one(temp);
                board |= temp;
            }
            board
        },
        Ray::SouthWest => {
            while (temp & RANK_1 == 0) && (temp & A_FILE == 0) {
                temp = south_one(temp);
                temp = west_one(temp);
                board |= temp;
            }
            board
        },
        Ray::West => {
            while temp & A_FILE == 0 {
                temp = west_one(temp);
                board |= temp;
            }
            board
        },
        Ray::NorthWest => {
            while (temp & RANK_8 == 0) && (temp & A_FILE == 0) {
                temp = north_one(temp);
                temp = west_one(temp);
                board |= temp;
            }
            board
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
