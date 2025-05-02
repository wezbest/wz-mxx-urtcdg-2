/*
ch28 - Copyable Values
*/

use crate::utils::{header, pswg};
use yansi::Paint;

////////////// Main Function calls//////////////////
pub fn kopy28_main() {
    pswg("38 - Copyable VAlues".to_string());
    learn1();
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

fn learn1() {
    let num = 69;

    // here a copy is being done implicitly, so there is no ownership value
    let other_num = num;

    println!("{}, {}", num.blue(), other_num.bold());
}
