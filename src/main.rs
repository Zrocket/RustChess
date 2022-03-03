use std::fmt;


#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

/// A Board contains everything necessary to calculate moves and evaluate a position
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Board {
    //BitBoard for each side
    bb_sides: [BitBoard; 2],
    //BitBoards for all pieces and each side
    bb_pieces: [[BitBoard; 6]; 2],
    //State contains all relevant information for evaluating a position outside the pieces.
    state: State,
}
impl Board {
    // Bit masks
    pub const EMPTY: u64 = 0x0000000000000000;
    pub const A_FILE: u64 = 0x8080808080808080;
    pub const B_FILE: u64 = 0x4040404040404040;
    pub const C_FILE: u64 = 0x2020202020202020;
    pub const D_FILE: u64 = 0x1010101010101010;
    pub const E_FILE: u64 = 0x808080808080808;
    pub const F_FILE: u64 = 0x404040404040404;
    pub const G_FILE: u64 = 0x202020202020202;
    pub const H_FILE: u64 = 0x101010101010101;
    pub const RANK_1: u64 = 0x00000000000000FF;
    pub const RANK_2: u64 = 0x000000000000FF00;
    pub const RANK_3: u64 = 0x0000000000FF0000;
    pub const RANK_4: u64 = 0x00000000FF000000;
    pub const RANK_5: u64 = 0x000000FF00000000;
    pub const RANK_6: u64 = 0x0000FF0000000000;
    pub const RANK_7: u64 = 0x00FF000000000000;
    pub const RANK_8: u64 = 0xFF00000000000000;
    pub const A1_H8_DIAGONAL: u64 = 0x8040201008040201;
    pub const H1_A8_ANTIDIAGONAL: u64 = 0x0102040810204080;
    pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
    pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;

    ///Shift board northwest one square
    fn northwest_one(board: u64) -> u64 {
        (board << 7) & !Board::H_FILE
    }
    ///Shift board north one square
    fn north_one(board: u64) -> u64 {
        board << 8
    }
    ///Shift board northeast one square
    fn northeast_one(board: u64) -> u64 {
        (board << 9) & !Board::A_FILE
    }
    ///Shift board west one square
    fn west_one(board: u64) -> u64 {
        (board >> 1) & !Board::H_FILE
    }
    ///Shift board east one square
    fn east_one(board: u64) -> u64 {
        (board << 1) & !Board::A_FILE
    }
    ///Shift board southwest one square
    fn southwest_one(board: u64) -> u64 {
        (board >> 9) & !Board::H_FILE
    }
    ///Shifrt board south one square
    fn south_one(board: u64) -> u64 {
        board >> 8
    }
    ///Shift board southeast one square
    fn southeast_one(board: u64) -> u64 {
        (board >> 7) & !Board::A_FILE
    }
    ///Rotate board left
    fn rotate_left(board: u64, s: i32) -> u64 {
        (board << s) | (board >> (64 - s))
    }
    ///Rotate board right
    fn rotate_right(board: u64, s: i32) -> u64 {
        (board >> s) | (board << (64 - s))
    }

}
impl Default for Board {
    fn default() -> Self {
    }
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
/// 0 1 0 1   0                 1                0                  1
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
    //Bit masks
    pub const WHITE_DEFAULT: u64 = Board::RANK_2;
    pub const BLACK_DEFAULT: u64 = Board::RANK_7;

    ///Push white pawn by one square
    fn white_push(pawn: u64, empty_squares: u64) -> u64 {
        Board::north_one(pawn) & empty_squares
    }
    ///Push white pawn by two squares
    fn white_double_push(pawn: u64, empty_squares: u64) -> u64 {
        let single_pushes: u64 = Pawn::white_push(pawn, empty_squares);
        Board::north_one(pawn) & empty_squares & Board::RANK_4
    }
    ///Push black pawn by one square
    fn black_push(pawn: u64, empty_squares: u64) -> u64 {
        Board::south_one(pawn) & empty_squares
    } 
    ///Push black pawn by two squares
    fn black_double_push(pawn: u64, empty_squares: u64) -> u64 {
        let single_pushes: u64 = Pawn::black_push(pawn, empty_squares);
        Board::south_one(pawn) & empty_squares & Board::RANK_4
    }
    ///White pawns that can be pushed
    fn white_pushable(white_pawns: u64, empty_squares: u64) -> u64 {
        Board::south_one(empty_squares) & white_pawns
    }
    ///White pawns that can be double pushed
    fn white_double_pushable(white_pawns: u64, empty_squares: u64) -> u64 {
        let empty_rank3 = Board::south_one(empty_squares & Board::RANK_4) & empty_squares;
        Pawn::white_pushable(white_pawns, empty_rank3)
    }
    ///Black pawns that can be pushed
    fn black_pushable(black_pawns: u64, empty_squares: u64) -> u64 {
        Board::north_one(empty_squares) & black_pawns
    }
    ///Black pawns that can be double pushed
    fn black_double_pushable(black_pawns: u64, empty_squares: u64) -> u64 {
        let empty_rank3 = Board::north_one(empty_squares & Board::RANK_4) & empty_squares;
        Pawn::black_pushable(black_pawns, empty_rank3)
    }
    ///White pawn east attacks
    fn white_east_attacks(white_pawns: u64) -> u64 {
        Board::northeast_one(white_pawns)
    }
    ///White pawn west attacks
    fn white_west_attacks(white_pawns: u64) -> u64 {
        Board::northeast_one(white_pawns)
    }
    ///Black pawn east attacks
    fn black_east_attacks(black_pawns: u64) -> u64 {
        Board::southeast_one(black_pawns)
    }
    ///Black pawn west attacks
    fn black_west_attacks(black_pawns: u64) -> u64 {
        Board::southwest_one(black_pawns)
    }
    ///All white pawn attacks
    fn white_any_attacks(white_pawns: u64) -> u64 {
        Pawn::white_west_attacks(white_pawns) | Pawn::white_east_attacks(white_pawns)
    }
    ///All black pawn attacks
    fn black_any_attacks(black_pawns: u64) -> u64 {
        Pawn::black_west_attacks(black_pawns) | Pawn::black_east_attacks(black_pawns)
    }
    ///White pawn double attacks
    fn white_double_attacks(white_pawns: u64) -> u64 {
        Pawn::white_east_attacks(white_pawns) & Pawn::white_west_attacks(white_pawns)
    }
    ///White pawn single attacks
    fn white_single_attacks(white_pawns: u64) -> u64 {
        Pawn::white_east_attacks(white_pawns) ^ Pawn::white_west_attacks(white_pawns)
    }
    ///Black pawn double attacks
    fn black_double_attacks(black_pawns: u64) -> u64 {
        Pawn::black_east_attacks(black_pawns) & Pawn::black_west_attacks(black_pawns)
    }
    ///Black pawn single attacks
    fn black_single_attacks(black_pawns: u64) -> u64 {
        Pawn::black_east_attacks(black_pawns) ^ Pawn::black_west_attacks(black_pawns)
    }

    fn white_safe_squares(white_pawns: u64, black_pawns: u64) -> u64 {
    }
    
    fn black_safe_squares(black_pawns: u64, white_pawns: u64) -> u64 {
    }

    fn white_capturable_east(white_pawns: u64, black_pawns: u64) -> u64 {
        Pawn::white_east_attacks(white_pawns) & black_pawns
    }
    fn white_capturable_west(white_pawns: u64, black_pawns: u64) -> u64 {
        Pawn::white_west_attacks(white_pawns) & black_pawns
    }
    fn white_capturable_any(white_pawns: u64, black_pawns: u64) -> u64 {
        Pawn::white_capturable_west(white_pawns, black_pawns) | Pawn::white_capturable_east(white_pawns, black_pawns)
    }

    fn black_capturable_east(black_pawns: u64, white_pawns: u64) -> u64 {
        Pawn::black_east_attacks(black_pawns) & white_pawns
    }
    fn black_capturable_west(black_pawns: u64, white_pawns: u64) -> u64 {
        Pawn::black_west_attacks(black_pawns) & white_pawns
    }
    fn black_capturable_any(black_pawns: u64, white_pawns: u64) -> u64 {
        Pawn::black_capturable_east(black_pawns, white_pawns) | Pawn::black_capturable_west(black_pawns, white_pawns)
    }
}

///Rook structure
pub struct Rook;
impl Rook {
    pub const WHITE_DEFAULT: u64 = (Board::RANK_1 & Board::A_FILE) | (Board::RANK_1 & Board::H_FILE);
    pub const BLACK_DEFAULT: u64 = (Board::RANK_8 & Board::A_FILE) | (Board::RANK_8 & Board::H_FILE);
}

///Knight structure
pub struct Knight;
impl Knight {
    pub const WHITE_DEFAULT: u64 = 0x42;
    pub const BLACK_DEFAULT: u64 = 0x4200000000000000;

    fn north_north_east(knight: u64) -> u64 {
        (knight << 15) & !Board::A_FILE
    }
    fn north_east_east(knight: u64) -> u64 {
        (knight << 10) & !(Board::A_FILE | Board::B_FILE)
    }
    fn south_east_east(knight: u64) -> u64 {
        (knight >> 6) & !(Board::A_FILE | Board::B_FILE)
    }
    fn south_south_east(knight: u64) -> u64 {
        (knight >> 15) & !Board::A_FILE
    }
    fn north_north_west(knight: u64) -> u64 {
        (knight << 17) & !Board::H_FILE
    }
    fn north_west_west(knight: u64) -> u64 {
        (knight << 6) & !(Board::G_FILE | Board::H_FILE)
    }
    fn south_south_west(knight: u64) -> u64 {
        (knight >> 17) & !Board::H_FILE
    }
    fn attacks(knights: u64) -> u64 {
        let east: u64 = Board::east_one(knights);
        let west: u64 = Board::west_one(knights);
        let attacks: u64 = ((east | west) << 16) | ((east | west) >> 16);
    }

}

///Bishop structure
pub struct Bishop;
impl Bishop {
    pub const WHITE_DEFAULT: u64 = 0x24;
    pub const BLACK_DEFAULT: u64 = 2400000000000000;
}

///Queen structure
pub struct Queen;
impl Queen {
    pub const WHITE_DEFAULT: u64 = 0x10;
    pub const BLACK_DEFAULT: u64 = 0x800000000000000;
}

///King structure
pub struct King;
impl King {
    pub const WHITE_DEFAULT: u64 = 0x8;
    pub const BLACK_DEFAULT: u64 = 0x1000000000000000;
}

fn main() {
    print!("Hello world!");
}
