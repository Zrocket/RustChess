pub trait Piece {
    fn moves(&self) -> u64; 

    fn attacks(&self, blockers: u64) -> u64;

    fn piece_square_value(&self) -> i32;

    fn board(&self) -> u64;
}
