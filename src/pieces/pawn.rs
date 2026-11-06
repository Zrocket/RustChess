use super::board;

/// Default board for white pawns
pub const WHITE_DEFAULT: u64 = board::RANK_2;
/// Default board for black pawns
pub const BLACK_DEFAULT: u64 = board::RANK_7;

pub const PAWN_TABLE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5,
    5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10,
    -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn piece_square_value(bboard: u64, side: board::Side) -> i32 {
    match side {
        board::Side::White => return PAWN_TABLE[bboard.trailing_zeros() as usize],
        board::Side::Black => {
            return PAWN_TABLE[63 - bboard.trailing_zeros() as usize]
        }
    }
}

/// Returns a bitboard of all valid moves
fn pawn_moves(bboard: u64, side: board::Side) -> u64 {
    pawn_push(bboard, side)
        | pawn_double_push(bboard, side)
}

/// Returns a bitboard of all valid attacks
fn pawn_attacks(bboard: u64, side: board::Side, blockers: u64) -> u64 {
    pawn_west_attacks(bboard, side)
        | pawn_east_attacks(bboard, side)
}

/// Return a bitboard of a side relevant pawn push
pub fn pawn_push(bboard: u64, side: board::Side) -> u64 {
    match side {
        board::Side::White => board::north_one(bboard),
        board::Side::Black => board::south_one(bboard),
    }
}
/// Returns a bitboard of a side relevant pawn double push
pub fn pawn_double_push(bboard: u64, side: board::Side) -> u64 {
    match side {
        board::Side::White => {
            if bboard & board::RANK_2 > 0 {
                board::north_one(pawn_push(bboard, side))
            } else {
                0
            }
        }
        board::Side::Black => {
            if bboard & board::RANK_7 > 0 {
                board::south_one(pawn_push(bboard, side))
            } else {
                0
            }
        }
    }
}

/// Returns a bitboard of valid west attacks
pub fn pawn_west_attacks(bboard: u64, side: board::Side) -> u64 {
    match side {
        board::Side::White => board::northwest_one(bboard),
        board::Side::Black => board::southwest_one(bboard),
    }
}

/// Returns a bitboard of valid east attacks
pub fn pawn_east_attacks(bboard: u64, side: board::Side) -> u64 {
    match side {
        board::Side::White => board::northeast_one(bboard),
        board::Side::Black => board::southeast_one(bboard),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pawn() {
    }
}
