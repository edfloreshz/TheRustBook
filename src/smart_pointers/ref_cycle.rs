use std::rc::Rc;
use std::cell::RefCell;
use crate::smart_pointers::ref_cycle::List::{RefCons, RefNil};

#[derive(Debug)]
pub enum List {
  RefCons(i32, RefCell<Rc<List>>),
  RefNil
}

impl List {
  pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      RefCons(_, item) => Some(item),
      RefNil => None,
    }
  }
}