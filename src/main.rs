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
    source: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(snake_info)
            .service(handle_move)
            .service(handle_end)
            .service(handle_start)
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

#[post("/start")]
async fn handle_start(start_req: web::Json<GameState>) -> HttpResponse {
    if &start_req.game.source == "league" {
        return HttpResponse::Ok().finish();
    }
    let snakes = &start_req.board.snakes
        .iter()
        .map(|s| &s.name)
        .collect::<Vec<&String>>();
    let game_id = &start_req.game.id;
    let game_mode = &start_req.game.map;
    let msg = format!("Game ID {} mode is {}. Snakes are {:?}", game_id, game_mode, snakes);
    if let Err(err) = ntfy_publish(msg).await {
        error!("Failed to post due to: {}", err);
    }
    HttpResponse::Ok().finish()
}

#[post("/end")]
async fn handle_end(end_req: web::Json<GameState>) -> HttpResponse {
    if &end_req.game.source == "league" {
        return HttpResponse::Ok().finish();
    }
    let snakes = &end_req.board.snakes;
    let winner = match snakes.iter().next() {
        Some(s) => &s.name,
        None => "No winner",
    };
    let game_id = &end_req.game.id;
    let msg = format!("Game ID {} winner was {}", game_id, winner);
    if let Err(err) = ntfy_publish(msg).await {
        error!("Failed to post due to: {}", err);
    }
    HttpResponse::Ok().finish()
}

async fn ntfy_publish(msg: String) -> Result<(), String> {
    let ntfy_server = match env::var("NTFY_SERVER") {
        Ok(val) => val,
        Err(_) => return Err(String::from("NTFY_SERVER env var not set"))
    };

    let client = reqwest::Client::new();
    match client.post(format!("https://ntfy.sh/{ntfy_server}"))
        .body(msg)
        .send()
        .await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}
