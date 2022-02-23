
///Pawn structure
pub struct Pawn;
impl Pawn {
    fn white_push(pawn: u64, empty: u64) -> u64 {
        pawn << 8 & empty
    }
    fn white_double_push(pawn: u64, empty: u64) -> u64 {
    }
    fn black_push(pawn: u64, empty: u64) -> u64 {
        pawn >> 8 & empty
    } 
    fn black_double_push(pawn: u64, empty: u64) -> u64 {
    }
}

///Knight structure
pub struct Knight;
impl Knight {
    fn north_north_west(knight: u64) -> u64 {
        knight << 17
    }
    fn north_east_east(knight: u64) -> u64 {
        knight << 10
    }
    fn south_east_east(knight: u64) -> u64 {
        knight >> 6
    }
    fn south_south_east(knight: u64) -> u64 {
        knight >> 15
    }
    fn north_north_west(knight: u64) -> u64 {
        knight << 15
    }
    fn north_west_west(knight: u64) -> u64 {
        knight 
    }
}
