use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpServer};
use env_logger::Env;
use log::info;
use serde::Deserialize;
use serde_json::{json, Value};

use battlesnake_rust::board::{Board, Coord};
use battlesnake_rust::snake::BattleSnake;

#[derive(Deserialize, Debug)]
pub struct GameState {
    game: Game,
    turn: u32,
    board: Board,
    you: BattleSnake,
}

#[derive(Deserialize, Debug)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(snake_info)
            .service(handle_move)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/")]
async fn snake_info() -> web::Json<Value> {
    info!("INFO");

    let response = json!({
        "apiversion": "1",
        "author": "jbkerry",
        "color": "#3300CC",
        "head": "cute-dragon",
        "tail": "offroad"
    });

    web::Json(response)
}

#[post("/move")]
async fn handle_move(move_req: web::Json<GameState>) -> web::Json<Value> {
    let response = move_req.you.determine_next_best_move(&move_req.board);
    web::Json(response)
}


