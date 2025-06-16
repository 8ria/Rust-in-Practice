#[allow(dead_code)]
mod intermediate {
    use std::collections::HashMap;
    use std::fmt;

    #[derive(Debug)]
    pub enum BankError {
        InsufficientFunds,
        InvalidAmount,
        AccountNotFound,
        AccountAlreadyExists,
    }

    impl fmt::Display for BankError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                BankError::InsufficientFunds => write!(f, "Insufficient funds"),
                BankError::InvalidAmount => write!(f, "Invalid amount"),
                BankError::AccountNotFound => write!(f, "Account not found"),
                BankError::AccountAlreadyExists => write!(f, "Account already exists"),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Transaction {
        pub transaction_type: TransactionType,
        pub amount: f64,
        pub timestamp: String,
    }

    #[derive(Debug, Clone)]
    pub enum TransactionType {
        Deposit,
        Withdrawal,
    }

    impl fmt::Display for TransactionType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                TransactionType::Deposit => write!(f, "Deposit"),
                TransactionType::Withdrawal => write!(f, "Withdrawal"),
            }
        }
    }

    #[derive(Debug)]
    pub struct BankAccount {
        account_number: String,
        owner_name: String,
        balance: f64,
        transaction_history: Vec<Transaction>,
    }

    impl BankAccount {
        pub fn new(account_number: String, owner_name: String, initial_balance: f64) -> Result<BankAccount, BankError> {
            if initial_balance < 0.0 {
                return Err(BankError::InvalidAmount);
            }
            
            Ok(BankAccount {
                account_number,
                owner_name,
                balance: initial_balance,
                transaction_history: Vec::new(),
            })
        }

        pub fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
            if amount <= 0.0 {
                return Err(BankError::InvalidAmount);
            }
            
            self.balance += amount;
            self.add_transaction(TransactionType::Deposit, amount);
            println!("Deposited ${:.2}. New balance: ${:.2}", amount, self.balance);
            Ok(())
        }

        pub fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
            if amount <= 0.0 {
                return Err(BankError::InvalidAmount);
            }
            if amount > self.balance {
                return Err(BankError::InsufficientFunds);
            }
            
            self.balance -= amount;
            self.add_transaction(TransactionType::Withdrawal, amount);
            println!("Withdrew ${:.2}. New balance: ${:.2}", amount, self.balance);
            Ok(())
        }

        fn add_transaction(&mut self, transaction_type: TransactionType, amount: f64) {
            let transaction = Transaction {
                transaction_type,
                amount,
                timestamp: "2024-01-01 12:00:00".to_string(), 
            };
            self.transaction_history.push(transaction);
        }

        pub fn get_balance(&self) -> f64 {
            self.balance
        }

        pub fn get_transaction_history(&self) -> &Vec<Transaction> {
            &self.transaction_history
        }

        pub fn display_info(&self) {
            println!("Account: {} | Owner: {} | Balance: ${:.2}", 
                     self.account_number, self.owner_name, self.balance);
        }

        pub fn display_transactions(&self) {
            println!("\n=== Transaction History for {} ===", self.account_number);
            for transaction in &self.transaction_history {
                println!("{}: ${:.2} at {}", 
                         transaction.transaction_type, transaction.amount, transaction.timestamp);
            }
        }
    }

    pub struct Bank {
        accounts: HashMap<String, BankAccount>,
    }

    impl Bank {
        pub fn new() -> Bank {
            Bank {
                accounts: HashMap::new(),
            }
        }

        pub fn create_account(&mut self, account_number: String, owner_name: String, initial_balance: f64) -> Result<(), BankError> {
            if self.accounts.contains_key(&account_number) {
                return Err(BankError::AccountAlreadyExists);
            }
            
            let account = BankAccount::new(account_number.clone(), owner_name, initial_balance)?;
            self.accounts.insert(account_number, account);
            println!("Account created successfully!");
            Ok(())
        }

        pub fn get_account_mut(&mut self, account_number: &str) -> Result<&mut BankAccount, BankError> {
            self.accounts.get_mut(account_number).ok_or(BankError::AccountNotFound)
        }

        pub fn get_account(&self, account_number: &str) -> Result<&BankAccount, BankError> {
            self.accounts.get(account_number).ok_or(BankError::AccountNotFound)
        }

        pub fn transfer(&mut self, from_account: &str, to_account: &str, amount: f64) -> Result<(), BankError> {
            
            if !self.accounts.contains_key(from_account) || !self.accounts.contains_key(to_account) {
                return Err(BankError::AccountNotFound);
            }
            
            {
                let from = self.accounts.get_mut(from_account).unwrap();
                from.withdraw(amount)?;
            }
            
            {
                let to = self.accounts.get_mut(to_account).unwrap();
                to.deposit(amount)?;
            }
            
            println!("Transferred ${:.2} from {} to {}", amount, from_account, to_account);
            Ok(())
        }

        pub fn list_accounts(&self) {
            println!("\n=== All Accounts ===");
            for account in self.accounts.values() {
                account.display_info();
            }
        }
    }

    pub fn run_demo() {
        println!("\n=== INTERMEDIATE VERSION ===");
        let mut bank = Bank::new();
        
        match bank.create_account("001".to_string(), "Alice Johnson".to_string(), 1000.0) {
            Ok(_) => println!("Alice's account created"),
            Err(e) => println!("Error: {}", e),
        }
        
        match bank.create_account("002".to_string(), "Bob Smith".to_string(), 500.0) {
            Ok(_) => println!("Bob's account created"),
            Err(e) => println!("Error: {}", e),
        }
        
        
        if let Ok(account) = bank.get_account_mut("001") {
            let _ = account.deposit(200.0);
            let _ = account.withdraw(150.0);
            account.display_transactions();
        }
        
        
        match bank.transfer("001", "002", 100.0) {
            Ok(_) => println!("Transfer successful"),
            Err(e) => println!("Transfer failed: {}", e),
        }
        
        bank.list_accounts();
    }
}

fn main() {
    intermediate::run_demo();
}
