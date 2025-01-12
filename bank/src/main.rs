#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
    name: String,
}

impl Bank {
    fn new(name: String) -> Self {
        Bank { accounts: Vec::new(), name }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
    
    fn set_account_balance(&mut self, id: u32, balance: f64) {
        for account in self.accounts.iter_mut() {
            if account.id == id {
                account.balance += balance;
                break;
            }
        }
    }

    fn get_account_balance(&self, id: u32) -> f64 {
        for account in self.accounts.iter() {
            if account.id == id {
                return account.balance;
            }
        }
        0.0
    }
}

#[derive(Debug)]
struct Account {
    id: u32,
    balance: f64,
    name: String,
}

impl Account {
    fn new(id: u32, name: String) -> Self {
        Account { id, balance: 0.0, name }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
}



fn main() {
    let mut bank = Bank::new("State Bank of India".to_string());

    let mut account1 = Account::new(bank.accounts.len() as u32 + 1, "John Doe".to_string());
    let mut account2 = Account::new(bank.accounts.len() as u32 + 2, "Jane Doe".to_string());
    
    account1.deposit(100.0);
    bank.add_account(account1);
    bank.add_account(account2);
    println!("Here is the bank: {:?}", bank);

    if let Some(account) = bank.accounts.get_mut(0) {
        account.deposit(100.0);
    }

    bank.set_account_balance(2, 100.0);

    println!("Here is the bank: {:?}", bank);

    let balance = bank.get_account_balance(2);
    println!("The balance of account 2 is: {}", balance);
    
}
