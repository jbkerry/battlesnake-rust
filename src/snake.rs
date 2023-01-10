use crate::board::Coord;

pub struct Snake {
    pub head: Coord
}

impl Snake {
    pub fn distance_to_food(&self, food: &Coord) -> u8 {
        self.head.x.abs_diff(food.x) + self.head.y.abs_diff(food.y)
    }

    pub fn move_towards_location(&self, location: &Coord) -> Vec<&str> {
        let mut moves: Vec<&str> = Vec::new();

        if location.x < self.head.x {
            moves.push("left");
        } else if location.x > self.head.x {
            moves.push("right");
        }

        if location.y < self.head.y {
            moves.push("down");
        } else if location.y > self.head.y {
            moves.push("up");
        }

        moves
    }
}