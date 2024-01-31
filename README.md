# Rust Banking System

This Rust program implements a basic banking system using Traits. It allows users to create accounts, deposit and withdraw money, and view their account balance.

## Overview

The program consists of the following components:

- **BankAccount Trait**: Defines operations for bank accounts such as deposit, withdraw, and balance.
- **Account Struct**: Represents a bank account with fields for account number and balance.
- **Implementation**: Implements the BankAccount trait for the Account struct, providing functionality for depositing, withdrawing, and checking balance.
- **Main Function**: Demonstrates the usage of the Account struct by creating an account, performing deposit and withdrawal operations, and checking the balance.

## Code Snippet

```rust
// Define a trait for bank operations
trait BankAccount {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), &'static str>;
    fn balance(&self) -> f64;
}

// Define a struct to represent a bank account
struct Account {
    account_number: u32,
    balance: f64,
}

// Implement the BankAccount trait for the Account struct
impl BankAccount for Account {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited {:.2} into account {}. New balance: {:.2}", amount, self.account_number, self.balance);
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), &'static str> {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrawn {:.2} from account {}. New balance: {:.2}", amount, self.account_number, self.balance);
            Ok(())
        } else {
            Err("Insufficient funds")
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create a new account
    let mut account = Account {
        account_number: 123456,
        balance: 1000.0,
    };

    println!("Account {} balance: {:.2}", account.account_number, account.balance());

    // Deposit some money
    account.deposit(500.0);

    // Withdraw some money
    match account.withdraw(200.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(err) => println!("Error: {}", err),
    }

    // Attempt to withdraw more money than available
    match account.withdraw(2000.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(err) => println!("Error: {}", err),
    }

    // Check balance
    println!("Account {} balance: {:.2}", account.account_number, account.balance());
}
```

## Usage

To use the program, simply run the provided Rust code. The main function demonstrates the functionality of the banking system by creating an account, depositing and withdrawing money, and checking the account balance.

## Cargo Commands

- **Running the Application**: Use `cargo run` to compile and run the Rust application.
- **Checking for Errors**: Use `cargo check` to check for errors in the code without building the application.
- **Building the Application**: Use `cargo build` to build the Rust application without running it.
