/// Represents a single square on the board.
/// # Representation
/// 1 is A1
/// 2 is B1
/// 64 is H8
///
/// Example
/// ```
/// ```
///
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Square(usize);

/// Labels for every ['Square'] on the board.
///
/// Example
/// ```
/// ```
///
#[repr(usize)]
#[rustfmt::skip]
pub enum SquareLabels {
    None,
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Side {
    White,
    Black
}

pub const EMPTY: u64                = 0x0000000000000000;
// Columns
pub const A_FILE: u64               = 0x8080808080808080;
pub const B_FILE: u64               = 0x4040404040404040;
pub const C_FILE: u64               = 0x2020202020202020;
pub const D_FILE: u64               = 0x1010101010101010;
pub const E_FILE: u64               = 0x808080808080808;
pub const F_FILE: u64               = 0x404040404040404;
pub const G_FILE: u64               = 0x202020202020202;
pub const H_FILE: u64               = 0x101010101010101;
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
