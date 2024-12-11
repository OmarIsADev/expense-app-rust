use std::io::stdin;

use crate::storage;
use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Expense {
    description: String,
    amount: f64,
    category: String,
    date: NaiveDate,
}

struct Details {
    total: f64,
    total_this_month: f64,
    average: f64,
    months: Vec<u32>,
}

pub fn spending_detials() {
    println!("Spending Details:");

    let expenses = storage::read_expenses();

    if expenses.is_empty() {
        println!("No expenses found");
        return;
    }

    let mut spendings = Details {
        total: 0.0,
        total_this_month: 0.0,
        average: 0.0,
        months: Vec::new(),
    };
    

    let this_month_expenses: Vec<&Expense> = expenses
        .iter()
        .filter(|expense: &&Expense| {
            NaiveDate::parse_from_str(expense.date.to_string().as_str(), "%Y-%m-%d")
                .unwrap()
                .month()
                == Local::now().month()
        })
        .collect::<Vec<_>>();

    for expense in this_month_expenses {
        spendings.total_this_month += expense.amount;
    }

    for expense in expenses.iter() {
        spendings.total += expense.amount;
    }

    for expense in expenses.iter() {
        let month = NaiveDate::parse_from_str(expense.date.to_string().as_str(), "%Y-%m-%d")
            .unwrap()
            .month();
        if !spendings.months.contains(&month) {
            spendings.months.push(month);
        }
    }

    spendings.average = spendings.total / (spendings.months.len() as f64);

    println!("Total spendings this month: {}", spendings.total_this_month);
    println!("Total spendings: {}", spendings.total);
    println!("Average monthly spendings: {}", spendings.average);

    // take month
    println!("Enter month to monitor: (leave it blank to cancel)");

    let mut month: String = String::new();
    stdin().read_line(&mut month).expect("Failed to read line");

    if month.trim().is_empty() {
        return;
    }

    let month: u32 = month.trim().parse().unwrap();

    if  1 > month || month > 12 {
        return;
    }

    let month_expenses: Vec<&Expense> = expenses
        .iter()
        .filter(|expense: &&Expense| {
            NaiveDate::parse_from_str(expense.date.to_string().as_str(), "%Y-%m-%d")
                .unwrap()
                .month()
                == month
        })
        .collect::<Vec<_>>();

    let total = month_expenses.iter().fold(0.0, |acc, expense| acc + expense.amount);

    println!("Total spendings in month {}: {}", month, total);
}

pub fn view_expenses() {
    let expenses = storage::read_expenses();

    if expenses.is_empty() {
        println!("No expenses found");
        return;
    }

    let mut index = 1;
    for expense in expenses {
        println!("Expense: {}", index);
        println!("Amount: {}", expense.amount);
        println!("Category: {}", expense.category);
        println!("Date: {}", expense.date);
        println!("Description: {}", expense.description);
        println!("");
        index += 1;
    }
}

pub fn add_expense() {
    println!("Amount: ");
    let mut amount = String::new();
    stdin().read_line(&mut amount).expect("Failed to read line");

    println!("Category: ");
    let mut category = String::new();
    stdin()
        .read_line(&mut category)
        .expect("Failed to read line");

    println!("Date: (Leave it blank for today)");
    let mut date = String::new();
    match stdin().read_line(&mut date) {
        Ok(_) => {
            if date.trim().is_empty() {
                date = chrono::offset::Local::now().format("%Y-%m-%d").to_string()
            }
        }
        Err(_) => date = chrono::offset::Local::now().format("%Y-%m-%d").to_string(),
    }

    println!("Description: ");
    let mut description = String::new();
    stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    let expense = Expense {
        amount: amount.trim().parse().unwrap(),
        category: category.trim().to_string(),
        date: NaiveDate::parse_from_str(date.trim(), "%Y-%m-%d")
            .unwrap_or(Local::now().date_naive()),
        description: description.trim().to_string(),
    };

    if storage::write_expenses(expense) == "Done" {
        println!("Expense added successfully");
        return;
    }

    println!("Failed to add expense");
}

pub fn modify_expense() {
    println!("Choose the expense you want to modify: (write the index or press enter to cancel)");

    let mut index = String::new();
    match stdin().read_line(&mut index) {
        Ok(_) => {
            if index.trim().is_empty() {
                return;
            }

            let expense = storage::get_expense(index.trim().parse().unwrap()).unwrap();

            loop {
                println!("Pick the field you want to modify:");
                println!("0. cancel");
                println!("1. Amount");
                println!("2. Category");
                println!("3. Date");
                println!("4. Description");

                let mut choice: String = String::new();
                stdin().read_line(&mut choice).expect("Failed to read line");

                let choice: u32 = choice.trim().parse().unwrap();
                if choice == 0 {
                    break;
                }
                match choice {
                    1 => {
                        println!("Amount: ");
                        let mut amount = String::new();
                        stdin().read_line(&mut amount).expect("Failed to read line");

                        let expense = Expense {
                            amount: amount.trim().parse().unwrap(),
                            category: String::from(expense.category.clone()),
                            date: expense.date.clone(),
                            description: String::from(expense.description.clone()),
                        };

                        if storage::modify_expense(index.trim().parse().unwrap(), expense) == "Done"
                        {
                            println!("Expense modified successfully");
                            break;
                        }
                    }
                    2 => {
                        println!("Category: ");
                        let mut category = String::new();
                        stdin()
                            .read_line(&mut category)
                            .expect("Failed to read line");
                        let expense = Expense {
                            amount: expense.amount,
                            category: category.trim().to_string(),
                            date: expense.date.clone(),
                            description: String::from(expense.description.clone()),
                        };
                        if storage::modify_expense(index.trim().parse().unwrap(), expense) == "Done"
                        {
                            println!("Expense modified successfully");
                            break;
                        }
                    }
                    3 => {
                        println!("Date: ");
                        let mut date = String::new();
                        stdin().read_line(&mut date).expect("Failed to read line");
                        let expense = Expense {
                            amount: expense.amount,
                            category: String::from(expense.category.clone()),
                            date: NaiveDate::parse_from_str(date.trim(), "%Y-%m-%d")
                                .unwrap_or(Local::now().date_naive()),
                            description: String::from(expense.description.clone()),
                        };
                        if storage::modify_expense(index.trim().parse().unwrap(), expense) == "Done"
                        {
                            println!("Expense modified successfully");
                            break;
                        }
                    }
                    4 => {
                        println!("Description: ");
                        let mut description = String::new();
                        stdin()
                            .read_line(&mut description)
                            .expect("Failed to read line");
                        let expense = Expense {
                            amount: expense.amount,
                            category: String::from(expense.category.clone()),
                            date: expense.date.clone(),
                            description: description.trim().to_string(),
                        };
                        if storage::modify_expense(index.trim().parse().unwrap(), expense) == "Done"
                        {
                            println!("Expense modified successfully");
                            break;
                        }
                    }
                    _ => println!("Invalid choice"),
                }

                println!("Are you sure about the index? IT IS INVALID!");
            }
        }
        Err(_) => println!("Invalid input! HOW DARE YOU?!"),
    }
}

pub fn delete_expense() {
    println!("Choose the expense you want to delete: (write the index or press enter to cancel)");

    let mut index = String::new();
    match stdin().read_line(&mut index) {
        Ok(_) => {
            if index.trim().is_empty() {
                return;
            }
            let index = index.trim().parse().unwrap();
            if storage::delete_expense(index) == "Done" {
                println!("Expense deleted successfully");
                return;
            }
            println!("Are you sure about the index? IT IS INVALID!");
        }
        Err(_) => println!("Invalid input! HOW DARE YOU?!"),
    }
}
