/*
Section 4  - Bank Project
- Thii is the same file from sec3/bank1.rs
*/

// Impors
use super::but::get_input_amount;
use crate::utils::{header, pswg};
use std::io;
use yansi::Paint;

//////// /// Main function call ////////////
pub fn bank1_main() {
    sb_main_bank();
}

////// Sub functions here //////////

// Test function
fn sbtest() {
    pswg("Sub Bank 1".to_string());
    header("Sub Function test")
}

//// Sec1 - Bank Project Work Here //////////

// * Main struct
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    // Account deposit function - receives an amount and adds it to the balance
    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    // Account withdraw function - receives an amount and subtracts it from the balance
    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    // summary acount function
    fn summary(&self) -> String {
        format!(
            "Account ID: {}, Holder: {}, Balance: {}",
            self.id, self.holder, self.balance
        )
    }

    // Summary colors

    fn summary_colors(&self) -> String {
        format!(
            "{}: {}, {}: {}, {}: {}",
            Paint::blue("Account ID").bold(),
            Paint::cyan(&self.id).bold(),
            Paint::blue("Holder").bold(),
            Paint::green(&self.holder).bold(),
            Paint::blue("Balance").bold(),
            Paint::yellow(&self.balance).bold()
        )
    }
}

// Bank Struct that holds the Account structs
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    // Total balane function of the accounts
    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    // Summary function of the bank
    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }

    // Summary colors
    fn summary_colors(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary_colors())
            .collect::<Vec<String>>()
    }
}

//// Utility Functions Here ////////////

//// Sub Function 1 Here ////////////

fn sb_main_bank() {
    pswg("Main Bank Function".to_string());
    header("Bank Project");

    let mut bank = Bank::new();
    let mut account = Account::new(1, "Alice".to_string());
    let mut account2 = Account::new(2, "Bob".to_string());
    let mut account3 = Account::new(3, "Charlie".to_string());
    let mut account4 = Account::new(4, "Dave".to_string());

    // Get input for deposit
    // let deposit_amount = get_input_amount();
    // Deposit and withdraw some money
    account.deposit(100);
    account.withdraw(5);

    account2.deposit(10);
    account2.withdraw(5);

    account3.deposit(200);
    account3.withdraw(5);

    account4.deposit(3000);
    account4.withdraw(5);

    bank.add_account(account);
    bank.add_account(account2);
    bank.add_account(account3);
    bank.add_account(account4);

    println!("Bank: {:#?}", bank.summary_colors());
    println!("Bank: {:#?}", bank.summary());
    println!("Total balance: {}", bank.total_balance());
}
