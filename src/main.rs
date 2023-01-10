use std::collections::HashMap;

use rust_package::board::{Coord, Board};
use rust_package::math;
use rust_package::snake::Snake;

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

fn main() {
    let num_one = 4;
    let num_two = 6;
    let result = math::sum_of_squares(num_one, num_two);
    println!("The result of {num_one}^2 + {num_two}^2 is {result}");
    let data = "abc";
    let mut s = data.to_string();
    s.push_str("def");
    println!("Value is {s}");
    let board = Board { height: 11, width: 11};
    let snake = Snake {
        head: Coord {x: 4, y: 6}
    };
    let food1 = Coord {x: 6, y: 0};
    println!("food out of bounds? {}", food1.is_out_of_bounds(&board));
    let d = snake.distance_to_food(&food1);
    println!("Snake's distance from food is {d}");
    let moves = snake.move_towards_location(&food1);
    println!("To get there, move {moves:?}");

    let mut numbers = vec![3.0, 1.0, 4.0, 8.0];
    let mut numbers2 = vec![-3.0, -0.2, -1.0, 5.0, 1.0];
    println!("median of numbers = {}", math::median(&mut numbers));
    println!("median of numbers2 = {}", math::median(&mut numbers2));

    println!("Pig Latin = {}", pig_latin("How are you today"));

    let mut store: HashMap<String, Vec<String>> = HashMap::new();
    store.insert(String::from("Accounts"), vec![String::from("Bobby")]);
    println!("store = {store:?}");
    match get_dept_employees("Fake Dept", &store) {
        Some(_) => {}
        None => {println!("Fake Dept does not exist so can't get employees")}
    };
    add_emp_to_dept("Abi", "Accounts", &mut store);
    handle_instruction("Add Amir to Sales", &mut store);
    println!("store after = {store:#?}");
    let accounts: Vec<String> = get_dept_employees("Accounts", &store)
        .expect("Accounts does not exist in this company");
    println!("Accounts = {accounts:?}");
}

fn pig_latin(sentence: &str) -> String {
    let mut pl_sentence = String::new();
    for word in sentence.split_whitespace() {
        let first_letter = &word[0..1];
        if VOWELS.contains(&first_letter) {
            pl_sentence.push_str(&format!("{word}-hay "));
        } else {
            let rest_of_word = &word[1..];
            pl_sentence.push_str(&format!("{rest_of_word}-{first_letter}ay "))
        }
    }
    pl_sentence
}

fn handle_instruction(instruction: &str, store: &mut HashMap<String, Vec<String>>) -> () {
    let parts: Vec<&str> = instruction.split_whitespace().collect();
    add_emp_to_dept(parts[1], parts[3], store);
}

fn add_emp_to_dept(employee: &str, department: &str, store: &mut HashMap<String, Vec<String>>) -> () {
    store
        .entry(department.to_string())
        .or_insert(vec![])
        .push(employee.to_string());
}

fn get_dept_employees(
    department: &str,
    store: &HashMap<String, Vec<String>>
) -> Option<Vec<String>> {
    let mut employees = store
        .get(department)?
        // .expect("No employees in this department")
        .clone();

    employees.sort();
    Some(employees)
}
