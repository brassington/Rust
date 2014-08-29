use std::rc::Rc;

pub enum Node {
  Cons(uint, Rc::new<Node>),
  Nil
}
