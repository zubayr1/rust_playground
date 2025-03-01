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

    fn get_account_by_id(&self, id: u32) -> Option<&Account> {
        self.accounts.iter().find(|a| a.id == id)
    }

    fn print_bank_name(&self, bank: &Bank) {
        println!("Bank name: {}", bank.name);
    }
}

fn set_bank_name(bank: &mut Bank, name: String) {
    bank.name = name;
}

fn get_bank_name(bank: &Bank) -> &str {
    &bank.name
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

    fn wrong_update_balance(&mut self, amount: f64) {
        let account = self;

        account.balance = amount;
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

fn change_account_name(account: &mut Account, name: String) {
    account.name = name;
}

fn read_account_name(account: &Account) -> &str {
    &account.name
}

fn main() {
    make_and_print_account();

    let mut account = make_and_return_account();
    println!("{:?}", account);

    account.wrong_update_balance(100.0);
    println!("{:?}", account);

    let mut bank = Bank::new("My Bank".to_string());
    bank.add_account(account);
    println!("{:?}", bank);

    // account.deposit(100.0);
    // println!("{:?}", account);

    // bank.get_account_balance(1);

    println!("{:?}", bank.get_summary());

    bank.print_bank_name(&bank);
    bank.print_bank_name(&bank);

    let name = get_bank_name(&bank);

    // set_bank_name(&mut bank, "New Bank".to_string());
    // println!("{:?}", bank);

    println!("{}", name);

    // OR

    set_bank_name(&mut bank, "New Bank".to_string());
    println!("{:?}", bank);

    let mut account1 = make_and_return_account();
    let name1 = read_account_name(&account1);
    
    change_account_name(&mut account1, "New Name".to_string());
    println!("{:?}", account1);
    // println!("{}", name1);

    let name1 = read_account_name(&account1);
    println!("{}", name1);
}
