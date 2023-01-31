use serde::Deserialize;
use std::collections::HashMap;

use crate::snake::BattleSnake;

#[derive(Deserialize, Debug)]
pub struct Board {
    width: i8,
    height: i8,
    food: Vec<Coord>,
    hazards: Vec<Coord>,
    snakes: Vec<BattleSnake>
}

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

#[derive(Copy, Clone, Deserialize, Debug, PartialEq)]
pub struct Coord {
    pub x: i8,
    pub y: i8
}


impl Coord {
    pub fn is_out_of_bounds(&self, board: &Board) -> bool {
        self.x == board.width || self.x < 0 || self.y == board.height || self.y < 0
    }

    pub fn get_surrounding_coords(&self) -> HashMap<String, Coord> {
        let mut surrounding: HashMap<String, Coord> = HashMap::new();
        surrounding.insert(String::from("right"), Coord{x: self.x + 1, y: self.y});
        surrounding.insert(String::from("left"), Coord{x: self.x - 1, y: self.y});
        surrounding.insert(String::from("up"), Coord{x: self.x, y: self.y + 1});
        surrounding.insert(String::from("down"), Coord{x: self.x, y: self.y - 1});
        
        surrounding
    }
}