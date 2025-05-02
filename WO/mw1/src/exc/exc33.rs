/*
Chapter 246 -
You will do all the exercises in this chapter using the same code.
*/

use crate::utils::{header, pswg};
use yansi::Paint;

////////////// Main Function calls//////////////////
pub fn exc33_main() {
    pswg("Chapter 33 - Excercises On references".to_string());
    ex1();
}

//// Sub functions here ////

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

// TODO: Create a new function called 'print_num_accounts'
// It should take in a Bank as an argument, then print out the number
// of accounts stored in the 'bank.accounts' vector
// To get the number of elements in a vector, you can use the '.len()'
// method like this: 'bank.accounts.len()'

fn ex1_print_num_accounts(bank: &Bank) {
    println!("----\n Acc NUm {} \n-----", bank.accounts.len().green());
}

fn ex1() {
    header("Exercise 1");

    let mut bank = Bank::new();
    let account1 = Account::new(1, String::from("me1"));
    let account2 = Account::new(1, String::from("me2"));

    bank.accounts.push(account1);
    bank.accounts.push(account2);

    // TODO: call 'print_num_accounts' here:
    ex1_print_num_accounts(&bank);

    // Notice the existing println statement here! 'main' needs to use the
    // 'bank' value in two locations, so we probably shouldn't move the
    // 'bank' value.
    println!("{:#?}", bank);
}
