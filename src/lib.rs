use serde::Deserialize;
use std::collections::HashMap;

pub mod board;
pub mod snake;

#[derive(Deserialize)]
pub struct Board {
    width: i8,
    height: i8,
    food: Vec<Coord>,
    hazards: Vec<Coord>,
    snakes: Vec<BattleSnake>
}

#[derive(Deserialize)]
pub struct BattleSnake {
    id: String,
    name: String,
    health: u8,
    body: Vec<Coord>,
    latency: String,
    head: Coord,
    length: u32,
    shout: Option<String>,
    #[serde(default = "empty_hashmap")]
    is_move_safe: HashMap<String, bool>,
}

fn empty_hashmap() -> HashMap<String, bool> {
    vec![
        (String::from("up"), false),
        (String::from("down"), false),
        (String::from("left"), false),
        (String::from("right"), false),
    ]
        .into_iter()
        .collect()
}

#[derive(Copy, Clone, Deserialize, PartialEq)]
pub struct Coord {
    x: i8,
    y: i8
}