/*
Chapter 24 - using the same bank example to learn the owbership
and borrowing concepts in rust
*/

use crate::utils::{header, pswg};
use yansi::Paint;

pub fn conc_main() {
    pswg("Chapter 24 - Ownership in Rust".to_string());
    // learn2();
    // learn3();
    // learn4();
    // learn5();
    // learn6();
    learn7();
}

///////////// Actual code starts here /////////////

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

fn learn1_print_account(account: Account) {
    println!("{:#?}", account.yellow());
}

fn learn1() {
    header("Ownership in Rust");

    let bank = Bank::new();
    // let account = Account::new(1, String::from("John Doe"));
    // Print the account
    // learn1_print_account(&account);

    let other_bank = bank;
    // println!("{:#?}", bank);
    println!("{:#?}", other_bank.yellow()); // <- Works 
}
// Code is for illustrationg values that get moved
fn learn2() {
    header("Ownershi in Rust 2");

    let account = Account::new(1, String::from("Pussy Licker"));

    learn1_print_account(account);
    // learn1_print_account(account);
}

// Case 3 - account value is being borrowed
fn learn3() {
    header("Case 3 - Ownership in Rust");

    // Account declared here
    let account = Account::new(3, String::from("BootyBoy"));

    // Then value is moved here
    let list_of_accounts = vec![account];

    // This function will fail since it is being borrowed to print
    // println!("{:#?}", account.yellow());
}

// Case 4 - Borrowed of move example
fn learn4() {
    header("Case 4 - Borrow of moved example");

    let bank = Bank::new();

    let account = bank.accounts;

    // println!("{:#?}", bank.accounts.yellow());
}

// Case 5 - Borrowing a value
fn learn5() {
    header("Case 5");

    let account = Account::new(5, String::from("WomanFartSmell"));

    // accounnt from l104 , gets moved here
    learn1_print_account(account);

    // This will fail since the value is moved
    // println!("{:#?}", account.holder.yellow());
}

// Case 6 - Talking about partial move. Which is also not allowed

fn learn6_print_holder(holder: String) {
    println!("{:#?}", holder.magenta());
}

fn learn6() {
    header("Cass 6 - Partial Move Error");

    let account = Account::new(6, String::from("PantySmeller"));

    learn6_print_holder(account.holder);

    // learn1_print_account(account);
}

// Resuming form Section 30

fn learn7_print_holder(holder: String) {
    println!("{:#?}", holder.magenta());
}

fn learn7_print_account(account: Account) -> Account {
    println!("{:#?}", account.yellow());
    account
}

fn learn7() {
    header("Learn 7 - Writing Useful Code with ownership");

    let mut account = Account::new(7, String::from("WomanSniffer"));

    account = learn7_print_account(account);
    account = learn7_print_account(account);

    println!("{:#?}", account.yellow());
}
