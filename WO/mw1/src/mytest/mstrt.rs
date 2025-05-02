/*
Actuakl Struct test
*/

#![allow(unused)]

use crate::utils::{header, pswg};
use yansi::Paint;

/////////// Main function ///////////
/*
Note: This main function is calling all the other subfunction
*/
pub fn mstrt_main() {
    // Printing the header text
    header("Structs and Implementation");

    // Call the test function
    struct_impl_test1();
}

#[derive(Debug)]
struct Woman {
    boobs: String,
    pussy: String,
    ass: String,
}

impl Woman {
    // fn sniff() -> Self {
    //     println!("Sniff her Ass");
    //     Self {
    //         boobs: String::from("Default"),
    //         pussy: String::from("Default"),
    //         ass: String::from("Default"),
    //     }
    // }

    fn lick(&self) -> String {
        format!(
            "Woman boobs {} and smelly {} giant {}",
            self.boobs, self.pussy, self.ass
        )
    }
}

fn struct_impl_test1() {
    pswg("Struct and Implementation Test".to_string());

    let woman1 = Woman {
        boobs: String::from("Big"),
        pussy: String::from("Hairy"),
        ass: String::from("fluffy"),
    };

    println!("{}", woman1.lick());
}
