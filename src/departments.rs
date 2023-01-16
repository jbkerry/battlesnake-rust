use std::collections::HashMap;

pub fn run_test_code() {
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

pub fn handle_instruction(instruction: &str, store: &mut HashMap<String, Vec<String>>) -> () {
    let parts: Vec<&str> = instruction.split_whitespace().collect();
    add_emp_to_dept(parts[1], parts[3], store);
}

fn add_emp_to_dept(employee: &str, department: &str, store: &mut HashMap<String, Vec<String>>) -> () {
    store
        .entry(department.to_string())
        .or_insert(vec![])
        .push(employee.to_string());
}

pub fn get_dept_employees(
    department: &str,
    store: &HashMap<String, Vec<String>>
) -> Option<Vec<String>> {
    let mut employees = store
        .get(department)?
        .clone();

    employees.sort();
    Some(employees)
}