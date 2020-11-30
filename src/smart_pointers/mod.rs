mod weak_ptr;
mod ref_cycle;
mod refcell_ptr;
mod drop;
mod box_ptr;
mod rc_ptr;

use box_ptr::{MyBox};
use rc_ptr::RcList;
use drop::CustomSmartPointer;
use crate::smart_pointers::box_ptr::List::{Cons, Nil};
use crate::smart_pointers::RcList::{RcCons, RcNil};
use crate::smart_pointers::ref_cycle::List::{RefCons, RefNil};

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::smart_pointers::weak_ptr::Node;

pub fn test_smrt_ptrs() {
  let _list = Cons(
    1,
    Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))),
  );
  let x = 1;
  let y = MyBox::new(x);
  println!("{}", *y);
  let m = MyBox::new(String::from("Rust"));
  hello(&m);
  // Drop
  let _c = CustomSmartPointer {data: String::from("Something")};
  // Reference counting
  let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
  println!("Ref count after creating a = {}", Rc::strong_count(&a));
  let _b = RcCons(3, Rc::clone(&a));
  println!("Ref count after creating b = {}", Rc::strong_count(&a));
  {
    let _c = RcCons(4, Rc::clone(&a));
    println!("Ref count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));

  // Reference cycle

  let a = Rc::new(RefCons(5, RefCell::new(Rc::new(RefNil))));
  println!("a initial rc count: {}", Rc::strong_count(&a));
  println!("a next item = {:?}", a.tail());

  let b = Rc::new(RefCons(10, RefCell::new(Rc::clone(&a))));
  println!("a rc count after b creation: {}", Rc::strong_count(&a));
  println!("b initial rc count: {}", Rc::strong_count(&b));
  println!("b next item = {:?}", b.tail());

  if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
  }

  println!("b rc count after changing a = {}", Rc::strong_count(&b));
  println!("a rc count after changing a = {}", Rc::strong_count(&a));

  // Uncomment to cause error
  // println!("a next item = {:?}", a.tail())

  // Weak<T>

  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![])
  });
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
  let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)])
  });

  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

  // Changes in Weak count
  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![])
  });
  println!(
    "leaf strong = {} weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
  );
  {
    let branch = Rc::new(Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
      "branch strong = {} weak = {}",
      Rc::strong_count(&branch),
      Rc::weak_count(&branch)
    );
    println!(
      "leaf strong = {} weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf)
    );

  }
  // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
  println!(
    "leaf strong = {} weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
  );
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

