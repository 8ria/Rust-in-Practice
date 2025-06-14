#[allow(dead_code)]
mod beginner {
    use std::collections::HashMap;

    pub struct BankAccount {
        pub account_number: String,
        pub owner_name: String,
        pub balance: f64,
    }

    impl BankAccount {
        pub fn new(account_number: String, owner_name: String, initial_balance: f64) -> BankAccount {
            BankAccount {
                account_number,
                owner_name,
                balance: initial_balance,
            }
        }

        pub fn deposit(&mut self, amount: f64) -> bool {
            if amount > 0.0 {
                self.balance += amount;
                println!("Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
                true
            } else {
                println!("Invalid deposit amount");
                false
            }
        }

        pub fn withdraw(&mut self, amount: f64) -> bool {
            if amount > 0.0 && amount <= self.balance {
                self.balance -= amount;
                println!("Withdrew ${:.2}. New balance: ${:.2}", amount, self.balance);
                true
            } else {
                println!("Invalid withdrawal amount or insufficient funds");
                false
            }
        }

        pub fn get_balance(&self) -> f64 {
            self.balance
        }

        pub fn display_info(&self) {
            println!("Account: {} | Owner: {} | Balance: ${:.2}", 
                     self.account_number, self.owner_name, self.balance);
        }
    }

    pub struct Bank {
        pub accounts: HashMap<String, BankAccount>,
    }

    impl Bank {
        pub fn new() -> Bank {
            Bank {
                accounts: HashMap::new(),
            }
        }

        pub fn create_account(&mut self, account_number: String, owner_name: String, initial_balance: f64) {
            let account = BankAccount::new(account_number.clone(), owner_name, initial_balance);
            self.accounts.insert(account_number, account);
            println!("Account created successfully!");
        }

        pub fn get_account(&mut self, account_number: &str) -> Option<&mut BankAccount> {
            self.accounts.get_mut(account_number)
        }

        pub fn list_accounts(&self) {
            println!("\n=== All Accounts ===");
            for account in self.accounts.values() {
                account.display_info();
            }
        }
    }

    pub fn run_demo() {
        println!("=== BEGINNER VERSION ===");
        let mut bank = Bank::new();
        
        bank.create_account("001".to_string(), "Alice Johnson".to_string(), 1000.0);
        bank.create_account("002".to_string(), "Bob Smith".to_string(), 500.0);
        
        if let Some(account) = bank.get_account("001") {
            account.deposit(200.0);
            account.withdraw(150.0);
        }
        bank.list_accounts();
    }
}

fn main() {
    beginner::run_demo();
}
