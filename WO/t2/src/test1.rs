/*
Various function tests
*/

#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::utz::print_with_synthwave_gradient;
use yansi::Paint;

pub fn test1_main() {
    print_with_synthwave_gradient("Multi Lines".to_string());
    subt1();
    subt2();
}

// Function to demonstrate multi-line text
fn subt1() {
    let multiline_text = r#"
Getting bacon to print multiple lines
is a pain the fuck. Fucekr deosnt want o print
- But when u do cargo run it will work
"#;

    println!("{}", multiline_text.to_string().green());
}

fn subt2() {
    let text = "This is a test";
    let text2 = "This is a test 2";
    let text3 = "This is a test 3";

    println!("{}", text.to_string().green());
    println!("{}", text2.to_string().green());
    println!("{}", text3.to_string().green());
}
