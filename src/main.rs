use std::fmt;


#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

/// A Board contains everything necessary to calculate moves and evaluate a position
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Board {
    //Board for each side
    bb_sides: [BitBoard; 2],
    //BitBoards for all pieces and each side
    bb_pieces: [[BitBoard; 6]; 2],
    //State contains all relevant information for evaluating a position outside the pieces.
    state: State,
}
impl Board {
    // Bit masks
    pub const A_FILE: u64 = 0xfefefefefefefefe;
    pub const H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
    pub const EMPTY: u64 = 0;
    pub const RANK_1: u64 = 0x00000000000000FF;
    pub const RANK_4: u64 = 0x00000000FF000000;
    pub const RANK_5: u64 = 0x000000FF00000000;
    pub const RANK_8: u64 = 0xFF00000000000000;
    pub const A1_H8_DIAGONAL: u64 = 0x8040201008040201;
    pub const H1_A8_ANTIDIAGONAL: u64 = 0x0102040810204080;
    pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
    pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;

    fn northwest_one(board: u64) -> u64 {
        (board << 7) & !Board::H_FILE
    }
    fn north_one(board: u64) -> u64 {
        board << 8
    }
    fn northeast_one(board: u64) -> u64 {
        (board << 9) & !Board::A_FILE
    }
    fn west_one(board: u64) -> u64 {
        (board >> 1) & !Board::H_FILE
    }
    fn east_one(board: u64) -> u64 {
        (board << 1) & !Board::A_FILE
    }
    fn southwest_one(board: u64) -> u64 {
        (board >> 9) & !Board::H_FILE
    }
    fn south_one(board: u64) -> u64 {
        board >> 8
    }
    fn southeast_one(board: u64) -> u64 {
        (board >> 7) & !Board::A_FILE
    }

    fn rotate_left(board: u64, s: i32) -> u64 {
        (board << s) | (board >> (64 - s))
    }
    fn rotate_right(board: u64, s: i32) -> u64 {
        (board >> s) | (board << (64 - s))
    }

}

/// Trait for piece movement shared methods
trait Moves {
    fn valid_moves(&self) -> u64;
    fn valid_attacks(&self) -> u64;
}

/// Side labels
pub struct Sides;
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}


/// Piece BitBoard index labels
pub struct Pieces;
impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

/// Contains castling_rights, move_clocks, en_passant_square if possible and the side to move
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    castling_rights: CastlingRights,
    en_passant_square: Option<Square>,
    half_move_counter: u8,
    stm: usize,
}

/// Castling rights are stored in a ['u8'], which is divided into the following parts:
/// ```text
/// 0 1 0 1   1                 1                0                  1
/// ^^^^^^^   ^                 ^                ^                  ^
/// unused    Black queen side  Black king side  White queen side   White king side
/// ```
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

///Provides labels for the ['CastlingRights']
pub struct Castling;
impl Castling {
    pub const NO_CASTLING: u8 = 0;
    pub const WHITE_00: u8 = 0b00000001;
    pub const WHITE_000: u8 = 0b00000010;
    pub const BLACK_00: u8 = 0b00000100;
    pub const BLACK_000: u8 = 0b00001000;

    pub const KING_SIDE: u8 = Self::BLACK_00 | Self::WHITE_00;
    pub const QUEEN_SIDE: u8 = Self::BLACK_000 | Self::WHITE_000;
    pub const WHITE_CASTLING: u8 = Self::WHITE_00 | Self::WHITE_000;
    pub const BLACK_CASTLING: u8 = Self::BLACK_00 | Self::BLACK_000;
    pub const ANY_CASTLING: u8 = Self::WHITE_CASTLING | Self::BLACK_CASTLING;
}

/// Represents a single square on the board.
/// # Representation
/// 1 is A1
/// 2 is B1
/// 64 is H8
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Square(usize);

/// Labels for every ['Square'] on the board.
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


///Pawn structure
pub struct Pawn;
impl Pawn {
    fn white_push(pawn: u64, empty_squares: u64) -> u64 {
        (pawn << 8) & empty_squares
    }
    fn white_double_push(pawn: u64, empty_squares: u64) -> u64 {
        let single_pushes: u64 = Pawn::white_push(pawn, empty_squares);
        (single_pushes << 8) & empty_squares & Board::RANK_4
    }
    fn black_push(pawn: u64, empty_squares: u64) -> u64 {
        pawn >> 8 & empty_squares
    } 
    fn black_double_push(pawn: u64, empty_squares: u64) -> u64 {
        let single_pushes: u64 = Pawn::black_push(pawn, empty_squares);
        (single_pushes >> 8) & empty_squares & Board::RANK_4
    }
    fn white_pushable(white_pawns: u64, empty_squares: u64) {
    }
    fn white_double_pushable(white_pawns: u64, empty_squares: u64) {
    }
    fn black_pushable(black_pawns: u64, empty_squares: u64) {
    }
    fn black_double_pushable(black_pawns: u64, empty_squares: u64) {
    }
}

///Knight structure
pub struct Knight;
impl Knight {
    fn north_north_west(knight: u64) -> u64 {
        knight << 17
    }
    fn north_east_east(knight: u64) -> u64 {
        knight << 10
    }
    fn south_east_east(knight: u64) -> u64 {
        knight >> 6
    }
    fn south_south_east(knight: u64) -> u64 {
        knight >> 15
    }
    fn north_north_west(knight: u64) -> u64 {
        knight << 15
    }
    fn north_west_west(knight: u64) -> u64 {
        knight 
    }
}

fn main() {
    print!("Hello world!");
}
