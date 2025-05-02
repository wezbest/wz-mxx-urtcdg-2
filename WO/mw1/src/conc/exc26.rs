/*
Chapter 246 -
You will do all the exercises in this chapter using the same code.
*/

use crate::utils::{header, pswg};
use yansi::Paint;

////////////// Main Function calls//////////////////
pub fn exc26_main() {
    pswg("Chapter 26 - Excercises".to_string());
    ex2();
}

////// Excercise Functions here //////////

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

// --- Excercise 1 ---

fn ex1_print_account(account: Account) {
    println!("{:#?}", account.green())
}

fn ex1() {
    header("Excercise 1");

    let account = Account::new(1, String::from("BootySniffer"));

    // TODO: Write and call a funcion that will *take ownership* of the account value, prin it, and return nothing

    // println!("{:#?}", account.green());
    ex1_print_account(account);

    // Can u call this function twice
    // println!("{:#?}", account.green());
}

// --- Excercise 2 ---

fn ex2_print_bank(bank: Bank) {
    println!("{:#?}", bank.green())
}

fn ex2_print_accounts(accounts: Vec<Account>) {
    println!("List of acounts  - {:#?}", accounts.green())
}

fn ex2() {
    header("Excercise 2");

    let bank = Bank::new();

    /*
    TODO: Write and call a function that will *take ownership* of the Banks "accounts" field, print it and return nothing
    */

    ex2_print_accounts(bank.accounts);

    /*
    Once you've finished the to-do, uncomment the print_bank call below. When your function + print_bank run, do you tink you'll end up getting an error?
    If so, what error do you think you'll get?
    */

    // ex2_print_bank(bank);

    /*
    If so, whar error do you think you'll get?
     */
}
