use std::collections::HashMap;

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

    pub fn get_surrounding_coords(&self) -> HashMap<&str, Coord> {
        let mut surrounding: HashMap<&str, Coord> = HashMap::new();
        surrounding.insert("right", Coord{x: self.x + 1, y: self.y});
        surrounding.insert("left", Coord{x: self.x - 1, y: self.y});
        surrounding.insert("up", Coord{x: self.x, y: self.y + 1});
        surrounding.insert("down", Coord{x: self.x, y: self.y - 1});
        
        surrounding
    }
}