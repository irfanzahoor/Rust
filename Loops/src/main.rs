// ===== Mini Project: ATM with Loops =====
use std::io;

fn main() {
    let mut balance: i32 = 1000; // starting balance

    println!("🏦 Welcome to Rust ATM!");
    println!("Your starting balance is: {}", balance);

    loop {
        // Menu
        println!("\n===== MENU =====");
        println!("1. Check Balance");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("4. Exit");
        println!("Enter your choice: ");

        // Input lena
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid input, please enter a number!");
                continue; // loop dobara start hoga
            }
        };

        // Conditions
        if choice == 1 {
            println!("💰 Your balance is: {}", balance);
        } else if choice == 2 {
            println!("Enter deposit amount:");
            let mut amount = String::new();
            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read input");
            let amount: i32 = amount.trim().parse().unwrap_or(0);
            balance += amount;
            println!("✅ Deposit successful! New balance = {}", balance);
        } else if choice == 3 {
            println!("Enter withdrawal amount:");
            let mut amount = String::new();
            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read input");
            let amount: i32 = amount.trim().parse().unwrap_or(0);

            if amount <= balance {
                balance -= amount;
                println!("✅ Withdrawal successful! New balance = {}", balance);
            } else {
                println!("❌ Insufficient funds!");
            }
        } else if choice == 4 {
            println!("👋 Thank you for using Rust ATM!");
            break; // loop se exit
        } else {
            println!("❌ Invalid choice, please try again!");
        }
    }
}
