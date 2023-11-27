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
        let mut rng = rand::thread_rng();

        Zobrist {
            pawn_white: (0..63).map(|_| rng.gen()).collect(),
            knight_white: (0..63).map(|_| rng.gen()).collect(),
            bishop_white: (0..63).map(|_| rng.gen()).collect(),
            rook_white: (0..63).map(|_| rng.gen()).collect(),
            queen_white: (0..63).map(|_| rng.gen()).collect(),
            king_white: (0..63).map(|_| rng.gen()).collect(),
            pawn_black: (0..63).map(|_| rng.gen()).collect(),
            knight_black: (0..63).map(|_| rng.gen()).collect(),
            bishop_black: (0..63).map(|_| rng.gen()).collect(),
            rook_black: (0..63).map(|_| rng.gen()).collect(),
            queen_black: (0..63).map(|_| rng.gen()).collect(),
            king_black: (0..63).map(|_| rng.gen()).collect(),
            move_black: rng.gen(),
            castling_king_black: rng.gen(),
            castling_queen_black: rng.gen(),
            castling_king_white: rng.gen(),
            castling_queen_white: rng.gen(),
            passant_a: rng.gen(),
            passant_b: rng.gen(),
            passant_c: rng.gen(),
            passant_d: rng.gen(),
            passant_e: rng.gen(),
            passant_f: rng.gen(),
            passant_g: rng.gen(),
            passant_h: rng.gen(),
        }
    }
}
