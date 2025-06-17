#[allow(dead_code)]
mod professional {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::fmt;
    use std::time::{SystemTime, UNIX_EPOCH};

    pub type Result<T> = std::result::Result<T, BankError>;
    pub type AccountId = String;
    pub type Money = u64; 

    #[derive(Debug, Clone, PartialEq)]
    pub enum BankError {
        InsufficientFunds { requested: Money, available: Money },
        InvalidAmount(Money),
        AccountNotFound(AccountId),
        AccountAlreadyExists(AccountId),
        ConcurrencyError(String),
    }

    impl fmt::Display for BankError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                BankError::InsufficientFunds { requested, available } => {
                    write!(f, "Insufficient funds: requested ${:.2}, available ${:.2}", 
                           *requested as f64 / 100.0, *available as f64 / 100.0)
                }
                BankError::InvalidAmount(amount) => write!(f, "Invalid amount: ${:.2}", *amount as f64 / 100.0),
                BankError::AccountNotFound(id) => write!(f, "Account not found: {}", id),
                BankError::AccountAlreadyExists(id) => write!(f, "Account already exists: {}", id),
                BankError::ConcurrencyError(msg) => write!(f, "Concurrency error: {}", msg),
            }
        }
    }

    impl std::error::Error for BankError {}

    #[derive(Debug, Clone)]
    pub struct Transaction {
        id: u64,
        transaction_type: TransactionType,
        amount: Money,
        timestamp: u64,
        description: Option<String>,
    }

    impl Transaction {
        fn new(transaction_type: TransactionType, amount: Money, description: Option<String>) -> Self {
            Self {
                id: Self::generate_id(),
                transaction_type,
                amount,
                timestamp: Self::current_timestamp(),
                description,
            }
        }

        fn generate_id() -> u64 {
            
            Self::current_timestamp()
        }

        fn current_timestamp() -> u64 {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        }

        pub fn amount_as_dollars(&self) -> f64 {
            self.amount as f64 / 100.0
        }
    }

    #[derive(Debug, Clone)]
    pub enum TransactionType {
        Deposit,
        Withdrawal,
        Transfer { from: AccountId, to: AccountId },
    }

    impl fmt::Display for TransactionType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                TransactionType::Deposit => write!(f, "Deposit"),
                TransactionType::Withdrawal => write!(f, "Withdrawal"),
                TransactionType::Transfer { from, to } => write!(f, "Transfer from {} to {}", from, to),
            }
        }
    }

    #[derive(Debug)]
    pub struct BankAccount {
        account_id: AccountId,
        owner_name: String,
        balance: Money,
        transaction_history: Vec<Transaction>,
        account_type: AccountType,
    }

    #[derive(Debug, Clone)]
    pub enum AccountType {
        Checking,
        Savings,
        Business,
    }

    impl BankAccount {
        pub fn new(account_id: AccountId, owner_name: String, initial_balance: Money, account_type: AccountType) -> Result<Self> {
            if initial_balance == 0 && matches!(account_type, AccountType::Business) {
                return Err(BankError::InvalidAmount(initial_balance));
            }

            Ok(Self {
                account_id,
                owner_name,
                balance: initial_balance,
                transaction_history: Vec::new(),
                account_type,
            })
        }

        pub fn deposit(&mut self, amount: Money, description: Option<String>) -> Result<()> {
            if amount == 0 {
                return Err(BankError::InvalidAmount(amount));
            }

            self.balance = self.balance.checked_add(amount)
                .ok_or_else(|| BankError::ConcurrencyError("Balance overflow".to_string()))?;

            let transaction = Transaction::new(TransactionType::Deposit, amount, description);
            self.transaction_history.push(transaction);

            println!("Deposited ${:.2}. New balance: ${:.2}", 
                     amount as f64 / 100.0, self.balance as f64 / 100.0);
            Ok(())
        }

        pub fn withdraw(&mut self, amount: Money, description: Option<String>) -> Result<()> {
            if amount == 0 {
                return Err(BankError::InvalidAmount(amount));
            }

            if amount > self.balance {
                return Err(BankError::InsufficientFunds { 
                    requested: amount, 
                    available: self.balance 
                });
            }

            self.balance -= amount;
            let transaction = Transaction::new(TransactionType::Withdrawal, amount, description);
            self.transaction_history.push(transaction);

            println!("Withdrew ${:.2}. New balance: ${:.2}", 
                     amount as f64 / 100.0, self.balance as f64 / 100.0);
            Ok(())
        }

        pub fn get_balance(&self) -> Money {
            self.balance
        }

        pub fn get_balance_as_dollars(&self) -> f64 {
            self.balance as f64 / 100.0
        }

        pub fn get_transactions(&self) -> &[Transaction] {
            &self.transaction_history
        }

        pub fn get_account_info(&self) -> AccountInfo {
            AccountInfo {
                account_id: self.account_id.clone(),
                owner_name: self.owner_name.clone(),
                balance: self.balance,
                account_type: self.account_type.clone(),
                transaction_count: self.transaction_history.len(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct AccountInfo {
        pub account_id: AccountId,
        pub owner_name: String,
        pub balance: Money,
        pub account_type: AccountType,
        pub transaction_count: usize,
    }

    impl fmt::Display for AccountInfo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Account: {} | Owner: {} | Type: {:?} | Balance: ${:.2} | Transactions: {}", 
                   self.account_id, self.owner_name, self.account_type, 
                   self.balance as f64 / 100.0, self.transaction_count)
        }
    }

    pub trait BankOperations {
        fn create_account(&self, account_id: AccountId, owner_name: String, initial_balance: Money, account_type: AccountType) -> Result<()>;
        fn deposit(&self, account_id: &str, amount: Money, description: Option<String>) -> Result<()>;
        fn withdraw(&self, account_id: &str, amount: Money, description: Option<String>) -> Result<()>;
        fn transfer(&self, from_account: &str, to_account: &str, amount: Money, description: Option<String>) -> Result<()>;
        fn get_account_info(&self, account_id: &str) -> Result<AccountInfo>;
        fn list_accounts(&self) -> Vec<AccountInfo>;
    }

    #[derive(Debug)]
    pub struct Bank {
        accounts: Arc<Mutex<HashMap<AccountId, BankAccount>>>,
        bank_name: String,
    }

    impl Bank {
        pub fn new(bank_name: String) -> Self {
            Self {
                accounts: Arc::new(Mutex::new(HashMap::new())),
                bank_name,
            }
        }

        fn dollars_to_cents(dollars: f64) -> Money {
            (dollars * 100.0).round() as Money
        }
    }

    impl BankOperations for Bank {
        fn create_account(&self, account_id: AccountId, owner_name: String, initial_balance: Money, account_type: AccountType) -> Result<()> {
            let mut accounts = self.accounts.lock()
                .map_err(|e| BankError::ConcurrencyError(format!("Lock poisoned: {}", e)))?;

            if accounts.contains_key(&account_id) {
                return Err(BankError::AccountAlreadyExists(account_id));
            }

            let account = BankAccount::new(account_id.clone(), owner_name, initial_balance, account_type)?;
            accounts.insert(account_id, account);
            println!("Account created successfully!");
            Ok(())
        }

        fn deposit(&self, account_id: &str, amount: Money, description: Option<String>) -> Result<()> {
            let mut accounts = self.accounts.lock()
                .map_err(|e| BankError::ConcurrencyError(format!("Lock poisoned: {}", e)))?;

            let account = accounts.get_mut(account_id)
                .ok_or_else(|| BankError::AccountNotFound(account_id.to_string()))?;

            account.deposit(amount, description)
        }

        fn withdraw(&self, account_id: &str, amount: Money, description: Option<String>) -> Result<()> {
            let mut accounts = self.accounts.lock()
                .map_err(|e| BankError::ConcurrencyError(format!("Lock poisoned: {}", e)))?;

            let account = accounts.get_mut(account_id)
                .ok_or_else(|| BankError::AccountNotFound(account_id.to_string()))?;

            account.withdraw(amount, description)
        }

        fn transfer(&self, from_account: &str, to_account: &str, amount: Money, description: Option<String>) -> Result<()> {
            let mut accounts = self.accounts.lock()
                .map_err(|e| BankError::ConcurrencyError(format!("Lock poisoned: {}", e)))?;

            
            if !accounts.contains_key(from_account) {
                return Err(BankError::AccountNotFound(from_account.to_string()));
            }
            if !accounts.contains_key(to_account) {
                return Err(BankError::AccountNotFound(to_account.to_string()));
            }
            
            {
                let from = accounts.get_mut(from_account).unwrap();
                from.withdraw(amount, description.clone())?;
            }

            {
                let to = accounts.get_mut(to_account).unwrap();
                to.deposit(amount, description)?;
            }

            println!("Transferred ${:.2} from {} to {}", 
                     amount as f64 / 100.0, from_account, to_account);
            Ok(())
        }

        fn get_account_info(&self, account_id: &str) -> Result<AccountInfo> {
            let accounts = self.accounts.lock()
                .map_err(|e| BankError::ConcurrencyError(format!("Lock poisoned: {}", e)))?;

            let account = accounts.get(account_id)
                .ok_or_else(|| BankError::AccountNotFound(account_id.to_string()))?;

            Ok(account.get_account_info())
        }

        fn list_accounts(&self) -> Vec<AccountInfo> {
            let accounts = self.accounts.lock().unwrap_or_else(|e| e.into_inner());
            accounts.values().map(|account| account.get_account_info()).collect()
        }
    }

    pub fn run_demo() {
        println!("\n=== PROFESSIONAL VERSION ===");
        let bank = Bank::new("Rust National Bank".to_string());

        let _ = bank.create_account("001".to_string(), "Alice Johnson".to_string(), 
                                  Bank::dollars_to_cents(1000.0), AccountType::Checking);
        let _ = bank.create_account("002".to_string(), "Bob Smith".to_string(), 
                                  Bank::dollars_to_cents(500.0), AccountType::Savings);
        let _ = bank.create_account("BIZ001".to_string(), "ACME Corp".to_string(), 
                                  Bank::dollars_to_cents(5000.0), AccountType::Business);

        let _ = bank.deposit("001", Bank::dollars_to_cents(200.0), Some("Salary deposit".to_string()));
        let _ = bank.withdraw("001", Bank::dollars_to_cents(150.0), Some("ATM withdrawal".to_string()));
        let _ = bank.transfer("001", "002", Bank::dollars_to_cents(100.0), Some("Monthly transfer".to_string()));

        println!("\n=== Account Information ===");
        for account_info in bank.list_accounts() {
            println!("{}", account_info);
        }
        
        match bank.withdraw("001", Bank::dollars_to_cents(10000.0), None) {
            Ok(_) => println!("Withdrawal successful"),
            Err(e) => println!("Withdrawal failed: {}", e),
        }
    }
}

fn main() {
    professional::run_demo();
}
