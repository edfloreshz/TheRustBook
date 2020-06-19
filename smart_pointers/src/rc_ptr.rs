use std::rc::Rc;

pub enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}