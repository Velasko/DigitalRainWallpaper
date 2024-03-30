#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl From<[usize; 2]> for Coord {
    fn from(value: [usize; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<Coord> for [usize; 2] {
    fn from(coord: Coord) -> [usize; 2] {
        [coord.x, coord.y]
    }
}
