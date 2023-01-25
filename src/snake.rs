use std::collections::HashMap;

use crate::board::{Board, Coord};

const ALLOWED_MOVES: [&str; 4] = ["up", "down", "left", "right"];

pub struct Snake<'a> {
    head: Coord,
    coords: HashMap<String, Coord>,
    is_move_safe: HashMap<&'a str, bool>
}

impl<'a> Snake<'a> {
    pub fn new(x: i8, y: i8) -> Self {
        let head: Coord = Coord{x, y};
        let coords = head.get_surrounding_coords();
        let mut is_move_safe: HashMap<&str, bool> = HashMap::new();
        for a_move in ALLOWED_MOVES {
            is_move_safe.insert(a_move, true);
        }
        Self {head, coords, is_move_safe}
    }

    pub fn determine_next_best_move(&mut self, board: &Board) -> HashMap<&str, &str> {
        let mut api_response = HashMap::new();
        self.ensure_does_not_go_out_of_bounds_or_hit_obstruction(board);
        for (direction, is_safe) in &self.is_move_safe {
            if *is_safe {
                api_response.insert("move", *direction);
                return api_response;
            }
        }
        api_response.insert("move", "up");
        api_response
    }

    pub fn distance_to_food(&self, food: &Coord) -> u8 {
        self.head.x.abs_diff(food.x) + self.head.y.abs_diff(food.y)
    }

    fn ensure_does_not_go_out_of_bounds_or_hit_obstruction(&mut self, board: &Board) {
        for snake_move in ALLOWED_MOVES {
            let coords = self.coords.get(snake_move).unwrap();
            self.is_move_safe.insert(snake_move, !coords.is_out_of_bounds(board));
        }
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