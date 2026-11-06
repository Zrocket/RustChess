use super::board;
use super::traits::Piece;

/// Default white king board
pub const WHITE_DEFAULT: u64 = 0x8;

/// Default black king board
pub const BLACK_DEFAULT: u64 = 0x1000000000000000;

pub const KING_TABLE: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
    -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40,
    -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20,
    30, 10, 0, 0, 10, 30, 20,
];

/// Returns all moves
fn moves(bboard: u64) -> u64 {

    board::north_one(bboard)
        | board::west_one(bboard)
        | board::east_one(bboard)
        | board::south_one(bboard)
        | board::northeast_one(bboard)
        | board::northwest_one(bboard)
        | board::southwest_one(bboard)
        | board::southeast_one(bboard)
}

/// Returns all attacks
fn attacks(bboard: u64, blockers: u64) -> u64 {
    todo!()
}

/// Returns the piece square value
fn piece_square_value(bboard: u64, side: board::Side) -> i32 {
    match side {
        board::Side::White => return KING_TABLE[bboard.trailing_zeros() as usize],
        board::Side::Black => {
            return KING_TABLE[64 - bboard.trailing_zeros() as usize]
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_king() {}
}
