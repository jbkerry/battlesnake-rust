pub fn run_test_code() {
    let num_one = 4;
    let num_two = 6;
    let result = sum_of_squares(num_one, num_two);
    println!("The result of {num_one}^2 + {num_two}^2 is {result}");

    let mut numbers = vec![3.0, 1.0, 4.0, 8.0];
    let mut numbers2 = vec![-3.0, -0.2, -1.0, 5.0, 1.0];
    println!("median of numbers = {}", median(&mut numbers));
    println!("median of numbers2 = {}", median(&mut numbers2));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn sum_of_squares(x: i32, y: i32) -> i32{
    add(square(x), square(y))
}

fn square(x: i32) -> i32 {
    x * x
}

pub fn median(numbers: &mut Vec<f64>) -> f64 {
    let length = numbers.len();
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if length % 2 != 0 {
        let median_position: f32 = (length as f32) / 2.0;
        let rounded = median_position.ceil() as usize;
        return numbers[rounded - 1]
    }
    let median_position = length / 2;
    let left = numbers[median_position - 1];
    let right = numbers[median_position];
    (left + right) / 2.0
}