/*
Work 1 - For this section on lifetimes
*/

use crate::utils::{header, pswg};
use yansi::Paint;

////// Main Entry function ///////

pub fn wo1_main() {
    pswg("Sectin 4 Lifetimes".to_string());
    // func2();
}

/////// Struct array and impl ///////
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

///// Sub Function1 //////

/*
Function is written to illustrato that after the function is over the values are dropped
*/

fn func1_makenprint() {
    let account = Account::new(1, String::from("ButyDanc"));

    println!("{:#?}", account.yellow());
}

fn func1() {
    header("Sub Function 1");
    func1_makenprint();
}

// Sub Function2

// fn func2_makenprint() -> &Account {
//     let account = Account::new(1, String::from("ButyDanc"));

//     println!("{:#?}", account.yellow());

//     &account
// }

// fn func2() {
//     header("Sub Function 2 - Actual example of lifetimes");

//     let acc_ref = func2_makenprint();

//     println!("{}", acc_ref.balance.cyan());
// }
