pub struct Board {
    pub width: i8,
    pub height: i8
}

pub struct Coord {
    pub x: i8,
    pub y: i8
}

impl Coord {
    pub fn is_out_of_bounds(&self, board: &Board) -> bool {
        self.x == board.width || self.x < 0 || self.y == board.height || self.y < 0
    }
}