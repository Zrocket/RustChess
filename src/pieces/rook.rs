use super::board::{self, RAY_EAST, RAY_NORTH, RAY_SOUTH, RAY_WEST};

/// Default white rook board
pub const WHITE_DEFAULT: u64 =
    (board::RANK_1 & board::A_FILE) | (board::RANK_1 & board::H_FILE);

/// Default black rook board
pub const BLACK_DEFAULT: u64 =
    (board::RANK_8 & board::A_FILE) | (board::RANK_8 & board::H_FILE);

pub const ROOK_TABLE: [i32; 64] = [
    0, 0, 0, 5, 5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10, 10, 10, 5,
    0, 0, 0, 0, 0, 0, 0, 0,
];

/// Return a bitboard of all valid moves
fn moves(bboard: u64) -> u64 {
    let index = board::bitscan(bboard).unwrap() as usize;
    RAY_NORTH[index] | RAY_EAST[index] | RAY_SOUTH[index] | RAY_WEST[index]
}

/// Return a bitboard of all vlalid attacks
fn attacks(bboard: u64, blockers: u64) -> u64 {
    todo!()
}

fn piece_square_value(bboard: u64, side: board::Side) -> i32 {
    match side {
        board::Side::White => ROOK_TABLE[bboard.trailing_zeros() as usize],
        board::Side::Black => ROOK_TABLE[64 - bboard.trailing_zeros() as usize],
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_rook() {}
}
