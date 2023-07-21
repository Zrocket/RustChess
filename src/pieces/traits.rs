use super::board;

pub trait Piece {
    fn moves(&self) -> Vec<board::Move>;

    fn attacks(&self, blockers: u64) -> Vec<board::Move>;

    fn piece_square_value(&self) -> i32;

    fn board(&self) -> u64;
}
