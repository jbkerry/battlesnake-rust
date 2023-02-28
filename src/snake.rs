use log::info;
use rand::seq::IteratorRandom;
use serde_json::{json, Value};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

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
        info!("Number of longer snake heads = {}", other_longer_snake_heads.len());
        // let mut lowest_possible_collisions: u8 = 5;
        // let mut direction = "down";
        let mut safety_of_moves = vec![];
        for (k, v) in is_move_safe.iter().filter(|&(k, v)| *v) {
            let surrounding_squares = coords.get(*k).unwrap().get_surrounding_coords();
            let mut counter: u8 = 0;
            for (_, v) in surrounding_squares.iter() {
                if other_longer_snake_heads.contains(v) {
                    counter += 1;
                }
            }
            let this_move_safety: (&str, u8) = (*k, counter);
            safety_of_moves.push(this_move_safety);
            // info!("{} got a count of {}", k, counter);
            // if counter < lowest_possible_collisions {
            //     direction = *k;
            //     lowest_possible_collisions = counter;
            // }
        }
        let safest_minimum = safety_of_moves.iter().min_by_key(|&&x| x.1).unwrap();
        let mut safest_moves: HashSet<&str> = safety_of_moves.iter()
            .filter(|&&x| x.1 == safest_minimum.1)
            .map(|&x| x.0).collect();
        // for (direction, is_safe) in is_move_safe {
        //     if is_safe {
        //         chosen_direction = String::from(direction);
        //         break;
        //     }
        // }
        if board.food.len() > 0 {
            let mut sorted_food = board.food.clone();
            sorted_food.sort_by(
                |a, b|
                    self.distance_to_food(a).cmp(&self.distance_to_food(b))
            );
            let nearest_food = &sorted_food[0];
            let moves_towards_food = self.move_towards_location(nearest_food);
            // let safest_moves_with_food: HashSet<_> = safest_moves.intersection(&moves_towards_food).collect();
             let safest_moves_with_food: HashSet<&str> = &safest_moves & &moves_towards_food;
            if safest_moves_with_food.len() > 0 {
                safest_moves = safest_moves_with_food
            }
        }
        let mut rng = rand::thread_rng();
        let direction = safest_moves.into_iter().choose(&mut rng).unwrap();
        info!("MOVE : {turn} - {direction}");
        json!({"move": direction})
    }

    pub fn distance_to_food(&self, food: &Coord) -> u8 {
        self.head.x.abs_diff(food.x) + self.head.y.abs_diff(food.y)
    }

    pub fn move_towards_location(&self, location: &Coord) -> HashSet<&str> {
        let mut moves: HashSet<&str> = HashSet::new();

        if location.x < self.head.x {
            moves.insert("left");
        } else if location.x > self.head.x {
            moves.insert("right");
        }

        if location.y < self.head.y {
            moves.insert("down");
        } else if location.y > self.head.y {
            moves.insert("up");
        }

        moves
    }
}