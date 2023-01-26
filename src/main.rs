use actix_web::{get, web, App, HttpServer};
use serde_json::Value;

use rust_battlesnake::board::Board;
use rust_battlesnake::snake::Snake;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_move)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/")]
async fn handle_move() -> web::Json<Value> {
    let board = Board {height: 11, width: 11};
    let mut snake = Snake::new(4, 6);
    let response = snake.determine_next_best_move(&board);
    web::Json(response)
}


