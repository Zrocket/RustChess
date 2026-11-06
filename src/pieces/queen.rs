use super::board::{
    self, RAY_EAST, RAY_NORTH, RAY_NORTH_EAST, RAY_NORTH_WEST, RAY_SOUTH, RAY_SOUTH_EAST,
    RAY_SOUTH_WEST, RAY_WEST,
};

/// Default white queen board
pub const WHITE_DEFAULT: u64 = 0x10;
/// Default black queen board
pub const BLACK_DEFAULT: u64 = 0x800000000000000;
pub const QUEEN_TABLE: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0,
    -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0,
    5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

/// Returns all moves
fn moves(bboard: u64) -> u64 {
    let index = bboard.trailing_zeros() as usize;

    RAY_NORTH[index]
        | RAY_EAST[index]
        | RAY_SOUTH[index]
        | RAY_WEST[index]
        | RAY_NORTH_EAST[index]
        | RAY_NORTH_WEST[index]
        | RAY_SOUTH_EAST[index]
        | RAY_SOUTH_WEST[index]
}

/// Returns all attacks
fn attacks(bboard: u64, pos: u64) -> u64 {
    todo!()
}

fn piece_square_value(bboard: u64, side: board::Side) -> i32 {
    match side {
        board::Side::White => return QUEEN_TABLE[bboard.trailing_zeros() as usize],
        board::Side::Black => {
            return QUEEN_TABLE[64 - bboard.trailing_zeros() as usize]
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_queen() {}
}
