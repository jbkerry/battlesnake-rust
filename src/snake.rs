use log::info;
use serde_json::{json, Value};
use serde::Deserialize;
use std::collections::HashMap;

use crate::board::{Board, Coord};

const ALLOWED_MOVES: [&str; 4] = ["up", "down", "left", "right"];

#[derive(Deserialize, Debug)]
pub struct BattleSnake {
    id: String,
    name: String,
    health: u8,
    pub body: Vec<Coord>,
    latency: String,
    head: Coord,
    length: u32,
    shout: Option<String>,
}

impl BattleSnake {
    pub fn determine_next_best_move(&self, board: &Board, turn: u32) -> Value {
        let mut chosen_direction = String::from("up");
        let coords = self.head.get_surrounding_coords();
        let mut is_move_safe: HashMap<&str, bool> = HashMap::new();

        for snake_move in ALLOWED_MOVES {
            let this_coord = coords.get(snake_move).unwrap();
            is_move_safe.insert(
                snake_move,
                this_coord.is_free(board)
            );
        }

        let mut other_longer_snake_heads: Vec<Coord> = Vec::new();
        for snake in &board.snakes {
            if snake.name != self.name && snake.length >= self.length {
                other_longer_snake_heads.push(snake.head);
            }
        }

        let lowest_possible_collisions: u8 = 5;
        let mut direction = "down";
        for (k, v) in is_move_safe.iter().filter(|&(k, v)| *v) {
            let surrounding_squares = coords.get(*k).unwrap().get_surrounding_coords();
            let mut counter: u8 = 0;
            for (_, v) in surrounding_squares.iter() {
                if other_longer_snake_heads.contains(v) {
                    counter += 1;
                }
            }
            if counter < lowest_possible_collisions {
                direction = *k;
            }
        }

        // for (direction, is_safe) in is_move_safe {
        //     if is_safe {
        //         chosen_direction = String::from(direction);
        //         break;
        //     }
        // }

        info!("MOVE : {turn} - {direction}");
        json!({"move": direction})
    }

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