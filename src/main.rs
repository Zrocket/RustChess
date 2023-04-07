use std::fmt;
use pieces::*;

pub mod pieces;

pub struct Game {
}

/// Contains castling_rights, move_clocks, en_passant_square if possible and the side to move
///
/// Example
/// ```
/// ```
///
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    castling_rights: CastlingRights,
    en_passant_square: Option<board::Square>,
    half_move_counter: u8,
    stm: usize,
}

/// Castling rights are stored in a ['u8'], which is divided into the following parts:
/// ```text
/// 0 1 0 1   0                 1                0                  1
/// ^^^^^^^   ^                 ^                ^                  ^
/// unused    Black queen side  Black king side  White queen side   White king side
/// ```
/// [short explanation of what the item does]
///
/// Example
/// ```
/// ```
///
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    fn empty() -> Self {
        Self(Castling::NO_CASTLING)
    }
    fn all() -> Self {
        Self::default()
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self(Castling::ANY_CASTLING)
    }
}

/// Provides labels for the ['CastlingRights']
///
/// Example
/// ```
/// ```
///
pub struct Castling;

impl Castling {
    pub const NO_CASTLING: u8   = 0;
    pub const WHITE_00: u8      = 0b00000001;
    pub const WHITE_000: u8     = 0b00000010;
    pub const BLACK_00: u8      = 0b00000100;
    pub const BLACK_000: u8     = 0b00001000;

    pub const KING_SIDE: u8         = Self::BLACK_00 | Self::WHITE_00;
    pub const QUEEN_SIDE: u8        = Self::BLACK_000 | Self::WHITE_000;
    pub const WHITE_CASTLING: u8    = Self::WHITE_00 | Self::WHITE_000;
    pub const BLACK_CASTLING: u8    = Self::BLACK_00 | Self::BLACK_000;
    pub const ANY_CASTLING: u8      = Self::WHITE_CASTLING | Self::BLACK_CASTLING;
}

fn main() {
    print!("Hello world!");
}
