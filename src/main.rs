use rust_package::math;

fn main() {
    let num_one = 4;
    let num_two = 6;
    let result = math::sum_of_squares(num_one, num_two);
    println!("The result of {num_one}^2 + {num_two}^2 is {result}");
}
