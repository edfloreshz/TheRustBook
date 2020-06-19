mod refcell_ptr;
mod drop;
mod box_ptr;
mod rc_ptr;

use box_ptr::{MyBox, List};
use rc_ptr::RcList;
use drop::CustomSmartPointer;
use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};

use std::rc::Rc;

fn main() {
    let list = Cons(
        1,
        Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))),
    );
    let x = 1;
    let y = MyBox::new(x);
    println!("{}", *y);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Drop
    let c = CustomSmartPointer {data: String::from("Something")};
    // Reference counting
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("Ref count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("Ref count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("Ref count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

