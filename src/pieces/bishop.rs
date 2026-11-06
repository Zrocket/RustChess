use super::board::{self, RAY_NORTH_EAST, RAY_NORTH_WEST, RAY_SOUTH_EAST, RAY_SOUTH_WEST};

/// Default white bishop board
pub const WHITE_DEFAULT: u64 = 0x24;

/// Default black bishop board
pub const BLACK_DEFAULT: u64 = 2400000000000000;

pub const BISHOP_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5,
    0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10,
    10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

/// Returns a bitboard of valid moves
fn moves(bboard: u64) -> u64 {
    let mut moves: u64 = 0;

    let index = board::bitscan(bboard).unwrap() as usize;
    let mut set = RAY_NORTH_EAST[index]
        | RAY_NORTH_WEST[index]
        | RAY_SOUTH_EAST[index]
        | RAY_SOUTH_WEST[index];

    while set != 0 {
        let to = set.trailing_zeros();
        moves |= to as u64;
        set |= set - 1;
    }

    moves
}

/// Returns a bitboard of valid attacks
fn attacks(bboard: u64, blockers: u64) -> u64 {
    todo!()
}

fn piece_square_value(bboard: u64, side: board::Side) -> i32 {
    match side {
        board::Side::White => {
            return BISHOP_TABLE[bboard.trailing_zeros() as usize]
        }
        board::Side::Black => {
            return BISHOP_TABLE[64 - bboard.trailing_zeros() as usize]
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_bishop() {}
}
