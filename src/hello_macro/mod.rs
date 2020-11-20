pub mod hello_macro;

use hello_macro::{HelloMacro};

struct Pancakes;

impl HelloMacro for Pancakes {
  fn hello_macro() {
    println!("Hello Macro!")
  }
}

pub fn test_macros() {
  Pancakes::hello_macro();
}