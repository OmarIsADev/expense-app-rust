use crate::expense::Expense;
use std::{
    fs::File,
    io::{Read, Write},
};

const FILE_PATH: &str = "expenses.json";

pub fn read_expenses() -> Vec<Expense> {
    // Code to read expenses from JSON file
    let mut file: File = {
        match File::open(FILE_PATH) {
            Ok(file) => file,
            Err(_) => File::create(FILE_PATH).expect("Failed to create file"),
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let data: Vec<Expense> = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };

    data
}

pub fn get_expense(index: usize) -> Option<Expense> {
    let expenses = read_expenses();
    if index <= expenses.len() {
        Some(expenses[index - 1].clone())
    } else {
        None
    }
}

pub fn write_expenses(data: Expense) -> &'static str {
    // Code to write expenses to JSON file
    let mut expense = read_expenses();
    expense.push(data);
    
    let json = serde_json::to_string(&expense).unwrap();
    
    let mut file = File::create(FILE_PATH).expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write file");

    "Done"
}

pub fn modify_expense(index: usize, data: Expense) -> &'static str {
    // Code to modify expenses in JSON file
    let mut expenses: Vec<Expense> = read_expenses();
    if index <= expenses.len() {
        expenses[index - 1] = data;
        let json: String = serde_json::to_string(&expenses).unwrap();
        let mut file: File = File::create(FILE_PATH).expect("Failed to create file");
        file.write_all(json.as_bytes())
            .expect("Failed to write file");
        "Done"
    }
    else {
        "Invalid index"
    }
}

pub fn delete_expense(index: usize) -> &'static str {
    // Code to delete expenses from JSON file

    let mut expenses = read_expenses();

    if index <= expenses.len() {
        expenses.remove(index - 1);
        let json = serde_json::to_string(&expenses).unwrap();
        let mut file = File::create(FILE_PATH).expect("Failed to create file");
        file.write_all(json.as_bytes())
            .expect("Failed to write file");
        "Done"
    }
    else {
        "Invalid index"
    }
}