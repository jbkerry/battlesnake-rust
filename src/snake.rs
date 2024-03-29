use log::{info, warn};
use rand::seq::IteratorRandom;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

use crate::{Board, Coord, BattleSnake};

const ALLOWED_MOVES: [&str; 4] = ["up", "down", "left", "right"];


impl BattleSnake {
    fn construct_safe_moves(&mut self, coords: &HashMap<String, Coord>, board: &Board) -> () {
        // let mut is_move_safe: HashMap<&str, bool> = HashMap::new();
        for snake_move in ALLOWED_MOVES {
        // for snake_move in self.is_move_safe.keys() {
            let this_coord = coords.get(snake_move).unwrap();
            let is_free: bool = this_coord.is_free(board);
            if is_free {
                self.is_move_safe.insert(
                    snake_move.to_string(),
                    is_free
                );
            }
        }
    }

    fn num_safe_moves(&self, safe_moves: &HashMap<String, bool>) -> Option<usize> {
        let num_safe_moves = safe_moves.values().filter(|&v| *v).count();
        match num_safe_moves {
            0 => None,
            value => Some(value),
        }
    }

    pub fn determine_next_best_move(&mut self, board: &Board, turn: u32) -> Value {
        let coords = self.head.get_surrounding_coords();
        self.construct_safe_moves(&coords, board);

        if self.num_safe_moves(&self.is_move_safe).is_none() {
            warn!("No safe moves! Moving down");
            info!("MOVE : {turn} - down");
            return json!({"move": "down"})
        }

        // choose least probable detrimental head-on collision

        let mut other_longer_snake_heads: Vec<Coord> = Vec::new();
        for snake in &board.snakes {
            if snake.name != self.name && snake.length >= self.length {
                other_longer_snake_heads.push(snake.head);
            }
        }

        let mut safety_of_moves = vec![];
        for (k, _) in self.is_move_safe.iter().filter(|&(_, v)| *v) {
            let surrounding_squares = coords.get(k).unwrap().get_surrounding_coords();
            let mut counter: u8 = 0;
            for (_, v) in surrounding_squares.iter() {
                if other_longer_snake_heads.contains(v) {
                    counter += 1;
                }
            }
            let this_move_safety: (&str, u8) = (k, counter);
            safety_of_moves.push(this_move_safety);
        }
        let safest_minimum = safety_of_moves.iter().min_by_key(|&&x| x.1).unwrap();
        let safest_moves: HashSet<&str> = safety_of_moves.iter()
            .filter(|&&x| x.1 == safest_minimum.1)
            .map(|&x| x.0).collect();


        // determine move towards largest open space

        let mut most_free: Vec<(&str, u32)> = vec![];
        for direction in &safest_moves {
            let mut available_branch_moves: Vec<Coord> = vec![*coords.get(*direction).unwrap()];
            let mut free_moves: u32 = 0;
            for idx in 0..100 {
                free_moves += 1;
                let next_move = match available_branch_moves.get(idx) {
                    Some(i) => i,
                    None => break,
                };
                if free_moves > self.length {
                    break;
                }
                for coord in next_move.get_surrounding_coords().values() {
                    if available_branch_moves.contains(coord) {
                        continue
                    }
                    let occupied: bool = board.obstructions().contains(coord);
                    if !occupied && !coord.is_out_of_bounds(board) {
                        available_branch_moves.push(*coord);
                    }
                }
            }
            most_free.push((direction, free_moves));
        }
        // info!("Free moves = {:?}", most_free);
        let max_free = most_free.iter().max_by_key(|&&x| x.1).unwrap();
        let mut safest_moves: HashSet<&str> = most_free.iter()
            .filter(|&&x| x.1 == max_free.1)
            .map(|&x| x.0).collect();


        // seek out food

        if board.food.len() > 0 {
            let mut sorted_food = board.food.clone();
            sorted_food.sort_by(
                |a, b|
                    self.distance_to_food(a).cmp(&self.distance_to_food(b))
            );
            let nearest_food = &sorted_food[0];
            let moves_towards_food = self.move_towards_location(nearest_food);
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