pub trait Piece {
    fn moves(&self) -> u64;

    fn attacks(&self, blockers: u64) -> u64;
}
