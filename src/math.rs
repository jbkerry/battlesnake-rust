fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn sum_of_squares(x: i32, y: i32) -> i32{
    add(square(x), square(y))
}

fn square(x: i32) -> i32 {
    x * x
}