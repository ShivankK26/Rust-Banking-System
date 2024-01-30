fn main() {
    let mut account = Account {
        account_number: 123456,
        balance: 1000.0,
    };

    println!(
        "Account {} balance: {:.2}",
        account.account_number,
        account.balance()
    );

    account.deposit(500.0);

    match account.withdraw(200.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(err) => println!("Error: {}", err),
    }

    match account.withdraw(2000.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(err) => println!("Error: {}", err),
    }

    println!(
        "Account {} balance: {:.2}",
        account.account_number,
        account.balance()
    );
}

trait BankAccount {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), &'static str>;
    fn balance(&self) -> f64;
}

struct Account {
    account_number: u32,
    balance: f64,
}

impl BankAccount for Account {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!(
            "Deposited {:.2} into account {}. New balance: {:.2}",
            amount, self.account_number, self.balance
        );
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), &'static str> {
        if self.balance >= amount {
            self.balance -= amount;
            println!(
                "Withdrawn {:.2} from account {}. New balance: {:.2}",
                amount, self.account_number, self.balance
            );
            Ok(())
        } else {
            Err("Insufficient funds")
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
