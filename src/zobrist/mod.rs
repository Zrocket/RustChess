use rand::Rng;

pub struct Zobrist {
    pawn_white: Vec<u64>,
    knight_white: Vec<u64>,
    bishop_white: Vec<u64>,
    rook_white: Vec<u64>,
    queen_white: Vec<u64>,
    king_white: Vec<u64>,
    pawn_black: Vec<u64>,
    knight_black: Vec<u64>,
    bishop_black: Vec<u64>,
    rook_black: Vec<u64>,
    queen_black: Vec<u64>,
    king_black: Vec<u64>,
    move_black: u64,
    castling_king_black: u64,
    castling_queen_black: u64,
    castling_king_white: u64,
    castling_queen_white: u64,
    passant_a: u64,
    passant_b: u64,
    passant_c: u64,
    passant_d: u64,
    passant_e: u64,
    passant_f: u64,
    passant_g: u64,
    passant_h: u64,
}

impl Default for Zobrist {
    fn default() -> Self {
        // Generate the zobrist
        Zobrist {
            pawn_white: (0..64).map(|_| rand::random()).collect(),
            knight_white: (0..64).map(|_| rand::random()).collect(),
            bishop_white: (0..64).map(|_| rand::random()).collect(),
            rook_white: (0..64).map(|_| rand::random()).collect(),
            queen_white: (0..64).map(|_| rand::random()).collect(),
            king_white: (0..64).map(|_| rand::random()).collect(),
            pawn_black: (0..64).map(|_| rand::random()).collect(),
            knight_black: (0..64).map(|_| rand::random()).collect(),
            bishop_black: (0..64).map(|_| rand::random()).collect(),
            rook_black: (0..64).map(|_| rand::random()).collect(),
            queen_black: (0..64).map(|_| rand::random()).collect(),
            king_black: (0..64).map(|_| rand::random()).collect(),
            move_black: rand::random(),
            castling_king_black: rand::random(),
            castling_queen_black: rand::random(),
            castling_king_white: rand::random(),
            castling_queen_white: rand::random(),
            passant_a: rand::random(),
            passant_b: rand::random(),
            passant_c: rand::random(),
            passant_d: rand::random(),
            passant_e: rand::random(),
            passant_f: rand::random(),
            passant_g: rand::random(),
            passant_h: rand::random(),
        }
    }
}
