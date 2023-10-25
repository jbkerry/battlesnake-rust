#![allow(dead_code, unused_variables)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use actix_web::{get, post, web, App, HttpServer, HttpResponse};
use env_logger::Env;
use log::{error, info};
use reqwest;
use serde::Deserialize;
use serde_json::{json, Value};

use battlesnake_rust::{Board, BattleSnake};

#[derive(Deserialize)]
pub struct GameState {
    game: Game,
    turn: u32,
    board: Board,
    you: RefCell<BattleSnake>,
}

#[derive(Deserialize)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
    map: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(snake_info)
            .service(handle_move)
            .service(handle_end)
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
        "color": "#B7410E",
        "head": "sleepy",
        "tail": "offroad"
    });

    web::Json(response)
}

#[post("/move")]
async fn handle_move(move_req: web::Json<GameState>) -> web::Json<Value> {
    let response = move_req.you.borrow_mut().determine_next_best_move(&move_req.board, move_req.turn);
    web::Json(response)
}

#[post("/end")]
async fn handle_end(end_req: web::Json<GameState>) -> HttpResponse {
    let ntfy_server = match env::var("NTFY_SERVER") {
        Ok(val) => val,
        Err(_) => {
            error!("NTFY_SERVER env var not set");
            return HttpResponse::NoContent().finish()
        }
    };

    let snakes = &end_req.board.snakes;
    let winner = match snakes.iter().next() {
        Some(s) => &s.name,
        None => "No winner",
    };
    let game_mode = &end_req.game.map;

    let client = reqwest::Client::new();
    match client.post(format!("https://ntfy.sh/{ntfy_server}"))
        .body(format!("Winner was {}; game mode was {}", winner, game_mode))
        .send().
        await {
            Ok(_) => {},
            Err(err) => error!("Failed to post due to: {}", err)
    }
    HttpResponse::Ok().finish()
}
