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
}

//// Utility Functions Here ////////////

//// Sub Function 1 Here ////////////

fn sb_main_bank() {
    pswg("Main Bank Function".to_string());
    header("Bank Project");

    let mut bank = Bank::new();
    let mut account = Account::new(1, "Alice".to_string());

    // Get input for deposit
    // let deposit_amount = get_input_amount();
    // Deposit and withdraw some money
    account.deposit(10);

    // Withdraw some money
    account.withdraw(5);

    bank.add_account(account);

    println!("Bank: {:#?}", bank.magenta());
}
