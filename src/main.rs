use rust_package::board::Board;
use rust_package::snake::Snake;

fn main() {
    let board = Board {height: 11, width: 11};
    let mut snake = Snake::new(4, 6);
    let next_move = snake.determine_next_best_move(&board);
    println!("API response is {next_move:#?}");
    // let food1 = Coord {x: 6, y: 0};
    // println!("food out of bounds? {}", food1.is_out_of_bounds(&board));
    // let d = snake.distance_to_food(&food1);
    // println!("Snake's distance from food is {d}");
    // let moves = snake.move_towards_location(&food1);
    // println!("To get there, move {moves:?}");
}



