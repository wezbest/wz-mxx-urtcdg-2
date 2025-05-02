/*
Section 3 - Bank Project work
*/

// Impors
use crate::utils::{header, pswg};

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
    balance: u32,
    id: u32,
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
}

// Helper function to print account
fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn sb_main_bank() {
    pswg("Main Bank Function".to_string());

    // Creating a new bank
    let bank = Bank::new();
    // Creating account in the new bank
    let account = Account::new(1, String::from("me"));

    // Printing the structs
    // println!("{:#?}", bank);
    print_account(account);
    print_account(account);
}
