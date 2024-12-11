mod expense;
mod storage;
use std::io::stdin;

fn main() {
    println!("Welcome to Expense Tracker!");

    loop {
        println!("--------------------------------");
        println!("| Enter 1 for spending details |");
        println!("| Enter 2 to view expenses     |");
        println!("| Enter 3 to add an expense    |");
        println!("| Enter 4 to modify an expense |");
        println!("| Enter 5 to delete an expense |");
        println!("| Enter 6 to quit              |");
        println!("--------------------------------");
        
        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read line");
        
        match choice.trim() {
            "1" => expense::spending_detials(),
            "2" => expense::view_expenses(),
            "3" => expense::add_expense(),
            "4" => expense::modify_expense(),
            "5" => expense::delete_expense(),
            "6" => break,
            _ => println!("Invalid choice"),
        }
    }
}