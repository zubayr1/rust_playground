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
        // for account in self.accounts.iter_mut() {
        //     if account.id == id {
        //         account.balance += balance;
        //         break;
        //     }
        // }

        self.accounts.iter_mut().find(|a| a.id == id).unwrap().balance = balance;
    }

    fn get_account_balance(&self, id: u32) -> f64 {
        for account in self.accounts.iter() {
            if account.id == id {
                return account.balance;
            }
        }
        0.0
    }

    fn get_total_balance(&self) -> f64 {
        self.accounts.iter().map(|a| a.balance).sum()
    }

    fn get_summary(&self) -> Vec<String> {
        self.accounts.iter().map(|a| format!("Account: {}, Balance: {}", a.id, a.balance)).collect()
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

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }
}

fn make_and_print_account() {
    let account = Account::new(1, "John Doe".to_string());
    println!("{:?}", account);
}

fn make_and_return_account() -> Account {
    let account = Account::new(2, "Jane Doe".to_string());
    account
}

fn main() {
    make_and_print_account();

    let mut account = make_and_return_account();
    println!("{:?}", account);

    let mut bank = Bank::new("My Bank".to_string());
    bank.add_account(account);
    println!("{:?}", bank);

    // account.deposit(100.0);
    // println!("{:?}", account);

    // bank.get_account_balance(1);

    println!("{:?}", bank.get_summary());
}
