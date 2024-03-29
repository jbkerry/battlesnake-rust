use std::collections::HashMap;

use crate::{Board, Coord};

impl Board {
    pub fn obstructions(&self) -> Vec<Coord> {
        let mut obstructions = Vec::new();
        for snake in &self.snakes {
            obstructions.extend(&snake.body[..snake.body.len() - 1]);
        }
        obstructions.extend(&self.hazards);
        obstructions
    }
}

impl Coord {
    pub fn is_out_of_bounds(&self, board: &Board) -> bool {
        self.x == board.width || self.x < 0 || self.y == board.height || self.y < 0
    }

    pub fn is_free(&self, board: &Board) -> bool {
        !self.is_out_of_bounds(board) && !board.obstructions().contains(self)
    }

    pub fn get_surrounding_coords(&self) -> HashMap<String, Coord> {
        let surrounding: HashMap<String, Coord> = vec![
            (String::from("right"), Coord{x: self.x + 1, y: self.y}),
            (String::from("left"), Coord{x: self.x - 1, y: self.y}),
            (String::from("up"), Coord{x: self.x, y: self.y + 1}),
            (String::from("down"), Coord{x: self.x, y: self.y - 1}),
        ]
            .into_iter()
            .collect();
        
        surrounding
    }
}


#[cfg(test)]
mod tests {
    use serde_json::{json, Value};
    use super::*;

    #[test]
    fn test_board_methods() {
        let board_json = json!({
            "width": 11,
            "height": 11,
            "food": [{"x": 4, "y": 5}],
            "hazards": [],
            "snakes": [
                {
                    "id": "1",
                    "name": "MrSnake",
                    "health": 90,
                    "body": [{"x": 3, "y": 2}, {"x": 3, "y": 1}, {"x": 3, "y": 0}],
                    "latency": "123",
                    "head": {"x": 3, "y": 2},
                    "length": 3,
                    "shout": ""
                },
                {
                    "id": "2",
                    "name": "MrsSnake",
                    "health": 97,
                    "body": [{"x": 9, "y": 9}, {"x": 8, "y": 9}],
                    "latency": "95",
                    "head": {"x": 9, "y": 9},
                    "length": 2,
                    "shout": ""
                },
            ]
        });
        let board: Board = serde_json::from_value(board_json).unwrap();
        let expected_obstructions = vec![
            Coord {x: 3, y: 2},
            Coord {x: 3, y: 1},
            Coord {x: 9, y: 9},
        ];
        assert_eq!(board.food.len(), 1);
        assert_eq!(board.obstructions(), expected_obstructions);
    }
}