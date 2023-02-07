pub const EMPTY: u16 = 0;
pub const WALL: u16 = 1;
pub const FOOD: u16 = 2;
pub const WYRM: u16 = 3;

#[must_use]
pub fn score(tile: u16) -> i8 {
    match tile {
        EMPTY => 0,
        FOOD => 1,
        _ => -1,
    }
}
