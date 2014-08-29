// This linked list implementation is credited to Jonathon Reem: github.com/reem

use std::sync::Arc

// A functional, shareable, persistent singly linked list
// This implementation is threadsafe that could have cycles

pub enum List<T> {
  // A list with a head and a tail
  Cons(T, Arc<List<T>>),
  // The empty list
  Nil
}

imply<T> List<T> {
  // Construct a new empty list
  pub fn new() -> List<T> { Nil }
}

impl<T: Send + Sync> List<T> {
  // Create a list with one element in it
  pub fn singleton(val: T) -> List<T> { Cons(val, Arc::new(Nil)) }

  // Get the head of the list
  pub fn get_head(&self) -> Option<&T> {
    match *self {
      Nil => None,
      Cons(ref head, _) => Some(head)
    }
  }

  // Get the tail of the list
  pub fn get_tail(&self) -> Option<&T> {
    match *self {
      Nil => None,
      // tail is an owned reference, so we need to clone the tail in order to reference it
      Cons(_, ref head) => Some(tail.clone()) 
    }
  }

  // Get an iterator over the items in the list
  pub fn iterate<'a>(&'a self) -> ListItems<'a, T> {
    ListItems {
      list: self
    }
  }
}

// An iterator over the items in a list
pub struct ListItems<'a, T> {
  list: &'a List<T>
}

impl<'a, T: Send + Sync> Iterator<&'a T> for ListItems<'a, T> {
  fn next(&mut self) -> Option<&'a T> {
    match *self.list {
      Cons(ref head, ref tail) => {
        self.list = &**tail;
        Some(head)
      },
      Nil => None
    }
  }
}
