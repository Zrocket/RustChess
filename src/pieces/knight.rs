use super::board;

/// Default white knight board
pub const WHITE_DEFAULT: u64 = 0x42;

/// Default black knight board
pub const BLACK_DEFAULT: u64 = 0x4200000000000000;

pub const KNIGHT_TABLE: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 5, 5, 0, -20, -40, -30, 5, 10, 15, 15,
    10, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 10, 15,
    15, 10, 0, -30, -40, -20, 0, 0, 0, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

pub fn north_north_east(bboard: u64) -> u64 {
    (bboard << 15) & !board::A_FILE
}

pub fn north_east_east(bboard: u64) -> u64 {
    (bboard << 10) & !(board::A_FILE | board::B_FILE)
}

pub fn south_east_east(bboard: u64) -> u64 {
    (bboard >> 6) & !(board::A_FILE | board::B_FILE)
}

pub fn south_south_east(bboard: u64) -> u64 {
    (bboard >> 15) & !board::A_FILE
}

pub fn north_north_west(bboard: u64) -> u64 {
    (bboard << 17) & !board::H_FILE
}

pub fn north_west_west(bboard: u64) -> u64 {
    (bboard << 6) & !(board::G_FILE | board::H_FILE)
}

pub fn south_south_west(bboard: u64) -> u64 {
    (bboard >> 17) & !board::H_FILE
}

/// Returns a bitboard of all valid moves in a Knight
fn moves(bboard: u64) -> u64 {
          north_north_east(bboard)
        | north_east_east(bboard)
        | south_east_east(bboard)
        | south_south_east(bboard)
        | north_north_west(bboard)
        | north_west_west(bboard)
        | south_south_west(bboard)
}

/// Returns a bitboard of all valid attacks
fn attacks(bboard: u64, blockers: u64) -> u64 {
    todo!()
}

fn piece_square_value(bboard: u64, side: board::Side) -> i32 {
    match side {
        board::Side::White => {
            return KNIGHT_TABLE[bboard.trailing_zeros() as usize]
        }
        board::Side::Black => {
            return KNIGHT_TABLE[64 - bboard.trailing_zeros() as usize]
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_knight() {}
}
