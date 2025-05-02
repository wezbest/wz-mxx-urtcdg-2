/*
Ch35 - Mutable References
*/

use crate::utils::{header, pswg};
use yansi::Paint;

////////////// Main Function calls//////////////////
pub fn mur35_main() {
    pswg("Chapter 35 - Mutable References".to_string());
    // learn1();
    learn2();
}

///////////////// Structs and Impl ////
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
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}
impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

////////////// learn1  Functions ////////////

fn learn1_print_account(account: &Account) {
    println!("{:#?}", account.yellow());
}

fn learn1_change_account(account: &mut Account) {
    account.balance += 100;
}

fn learn1() {
    header("L1 - Regarding Mutable References");

    let mut account = Account::new(1, String::from("Func Onner"));

    let account_ref = &mut account;

    // learn1_change_account(&mut account);

    // println!("Account after change: {:#?}", account.yellow());
    println!("Account after change: {:#?}", account_ref.holder.yellow());
}

//// Function 2 ////

fn learn2_print_account(account: &Account) {
    println!("{:#?}", account.yellow());
}

fn learn2_change_account(account: &mut Account) {
    account.balance += 100;
}

fn learn2() {
    header("L2 - Regarding Mutable References");

    let mut account = Account::new(1, String::from("Func Onner"));

    account.balance += 100;

    println!("Account before change: {:#?}", account.yellow());
}
