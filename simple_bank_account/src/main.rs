use std::vec::Vec;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum AccountType {
    Checking,
    Savings { interest_rate: f64 },
}

#[derive(Debug)]
enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer { to_account: String },
    ReceivedTransfer { from_account: String },
}

#[derive(Debug)]
struct Transaction {
    transaction_type: TransactionType,
    amount: f64,
    timestamp: u64,
}

#[derive(Debug)]
struct Account {
    holder_name: String,
    balance: f64,
    account_number: String,
    account_type: AccountType,
    transactions: Vec<Transaction>,
}

impl Account {
    fn new(name: String, initial_deposit: f64, account_number: String, account_type: AccountType) -> Account {
        let initial_transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            amount: initial_deposit,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        Account {
            holder_name: name,
            balance: initial_deposit,
            account_number: account_number,
            account_type,
            transactions: vec![initial_transaction],
        }
    }

    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(String::from("Deposit amount must be positive"));
        }

        self.balance += amount;
        self.transactions.push(Transaction {
            transaction_type: TransactionType::Deposit,
            amount,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(String::from("Withdrawal amount must be positive"));
        }
        if amount > self.balance {
            return Err(String::from("Insufficient funds"));
        }

        self.balance -= amount;
        self.transactions.push(Transaction {
            transaction_type: TransactionType::Withdrawal,
            amount,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        Ok(())
    }

    fn transfer(&mut self, to_account: &mut Account, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(String::from("Transfer amount must be positive"));
        }
        if amount > self.balance {
            return Err(String::from("Insufficient funds for transfer"));
        }

        // Record transfer in sender's account
        self.balance -= amount;
        self.transactions.push(Transaction {
            transaction_type: TransactionType::Transfer {
                to_account: to_account.account_number.clone(),
            },
            amount,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });

        // Record transfer in receiver's account
        to_account.balance += amount;
        to_account.transactions.push(Transaction {
            transaction_type: TransactionType::ReceivedTransfer {
                from_account: self.account_number.clone(),
            },
            amount,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });

        Ok(())
    }

    fn apply_interest(&mut self) -> Result<(), String> {
        match self.account_type {
            AccountType::Savings { interest_rate } => {
                let interest = self.balance * interest_rate;
                self.deposit(interest)?;
                Ok(())
            }
            AccountType::Checking => Err(String::from("Cannot apply interest to checking account")),
        }
    }

    fn display_info(&self) {
        println!("Account holder: {}", self.holder_name);
        println!("Account number: {}", self.account_number);
        println!("Account type: {}", match self.account_type {
            AccountType::Checking => "Checking",
            AccountType::Savings { interest_rate } => "Savings",
        });
        println!("Current balance: ${:.2}", self.balance);
    }

    fn display_transaction_history(&self) {
        println!("\nTransaction History for Account {}:", self.account_number);
        for transaction in &self.transactions {
            let transaction_type = match &transaction.transaction_type {
                TransactionType::Deposit => "Deposit".to_string(),
                TransactionType::Withdrawal => "Withdrawal".to_string(),
                TransactionType::Transfer { to_account } => 
                    format!("Transfer to account {}", to_account),
                TransactionType::ReceivedTransfer { from_account } => 
                    format!("Received transfer from account {}", from_account),
            };
            println!("Type: {}, Amount: ${:.2}, Timestamp: {}", 
                    transaction_type, transaction.amount, transaction.timestamp);
        }
    }
}

fn main() {
    // Create a checking account
    let mut checking = Account::new(
        String::from("John Doe"),
        1000.0,
        String::from("CHK-12345"),
        AccountType::Checking,
    );

    // Create a savings account
    let mut savings = Account::new(
        String::from("John Doe"),
        2000.0,
        String::from("SAV-67890"),
        AccountType::Savings { interest_rate: 0.05 },
    );

    // Test some transactions
    checking.deposit(500.0).unwrap();
    checking.withdraw(200.0).unwrap();
    checking.transfer(&mut savings, 300.0).unwrap();

    // Apply interest to savings
    savings.apply_interest().unwrap();

    // Display final state of both accounts
    println!("\nChecking Account:");
    checking.display_info();
    checking.display_transaction_history();

    println!("\nSavings Account:");
    savings.display_info();
    savings.display_transaction_history();
}