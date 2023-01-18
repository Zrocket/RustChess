pub trait Piece {
    fn moves(&self) -> u64;

    fn bboard(&self) -> u64;

    fn attacks(&self) -> u64;
}
